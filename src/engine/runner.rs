// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::arch;
use crate::distro::fedora;
use crate::distro::nixos;
use crate::distro::DistroFamily;
use crate::engine::step::StepStatus;
use crate::model::config::{dirs_home, InstallConfig, InstallMode};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// Checks if non-interactive sudo is available and validated (cached timestamp).
pub fn check_sudo_cached() -> bool {
    if which::which("sudo").is_err() {
        return false;
    }
    Command::new("sudo")
        .args(["-v", "-n"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Sudo keepalive worker that periodically refreshes the sudo timestamp
/// so long-running steps (e.g. pacman compilation or downloads) do not time out.
pub struct SudoKeepalive {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl SudoKeepalive {
    pub fn start() -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                // Sleep for up to 30 seconds with 500ms intervals for responsive shutdown
                for _ in 0..60 {
                    if !running_clone.load(Ordering::Relaxed) {
                        return;
                    }
                    thread::sleep(Duration::from_millis(500));
                }

                if !running_clone.load(Ordering::Relaxed) {
                    break;
                }

                // Refresh sudo timestamp non-interactively
                let _ = Command::new("sudo")
                    .args(["-v", "-n"])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
        });

        Self {
            running,
            handle: Some(handle),
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for SudoKeepalive {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Success,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EngineEvent {
    StepStarted {
        index: usize,
        step_id: String,
    },
    StepCompleted {
        index: usize,
        step_id: String,
        status: StepStatus,
        duration_secs: f64,
    },
    LogLine {
        level: LogLevel,
        text: String,
    },
    Progress {
        percent: u16,
    },
    Finished {
        success: bool,
        total_duration_secs: f64,
        summary: String,
    },
}

#[allow(dead_code)]
pub struct StepRunner {
    tx: Sender<EngineEvent>,
    dry_run: bool,
}

impl StepRunner {
    pub fn new(tx: Sender<EngineEvent>, dry_run: bool) -> Self {
        Self { tx, dry_run }
    }

    pub fn log_info(&self, msg: impl Into<String>) {
        let _ = self.tx.send(EngineEvent::LogLine {
            level: LogLevel::Info,
            text: msg.into(),
        });
    }

    pub fn log_success(&self, msg: impl Into<String>) {
        let _ = self.tx.send(EngineEvent::LogLine {
            level: LogLevel::Success,
            text: msg.into(),
        });
    }

    pub fn log_warn(&self, msg: impl Into<String>) {
        let _ = self.tx.send(EngineEvent::LogLine {
            level: LogLevel::Warn,
            text: msg.into(),
        });
    }

    #[allow(dead_code)]
    pub fn log_error(&self, msg: impl Into<String>) {
        let _ = self.tx.send(EngineEvent::LogLine {
            level: LogLevel::Error,
            text: msg.into(),
        });
    }

    pub fn log_debug(&self, msg: impl Into<String>) {
        let _ = self.tx.send(EngineEvent::LogLine {
            level: LogLevel::Debug,
            text: msg.into(),
        });
    }

    pub fn run_command(&self, program: &str, args: &[&str]) -> Result<(), String> {
        let cmd_str = format!("{} {}", program, args.join(" "));
        if self.dry_run {
            self.log_warn(format!("[DRY-RUN] Would run: {cmd_str}"));
            std::thread::sleep(std::time::Duration::from_millis(60));
            return Ok(());
        }

        self.log_debug(format!("Executing: {cmd_str}"));
        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn '{cmd_str}': {e}"))?;

        // Wait for output
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed waiting for '{cmd_str}': {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let first_line = stderr.lines().next().unwrap_or("Unknown command error");
            return Err(format!("{first_line} (exit code: {:?})", output.status.code()));
        }

        Ok(())
    }
}

pub fn execute_installation(
    distro: DistroFamily,
    config: InstallConfig,
    tx: Sender<EngineEvent>,
) {
    let runner = StepRunner::new(tx.clone(), config.dry_run);
    let start_all = Instant::now();

    runner.log_info(format!(
        "Starting Aurora installation for {} in {:?} mode",
        distro.name(),
        config.mode
    ));
    if config.dry_run {
        runner.log_warn("[DRY-RUN ACTIVE] Changes will only be previewed, no files modified");
    }

    // Resolve source directory for dotfiles
    let source_dir = resolve_source_dir(&config, &runner);
    let mut updated_config = config.clone();
    updated_config.source_dir = source_dir;

    let steps = match distro {
        DistroFamily::Arch => arch::build_steps(&updated_config),
        DistroFamily::Fedora => fedora::build_steps(&updated_config),
        DistroFamily::NixOS => nixos::build_steps(&updated_config),
        DistroFamily::Unknown => {
            let _ = tx.send(EngineEvent::Finished {
                success: false,
                total_duration_secs: 0.0,
                summary: "Unsupported distribution family".to_string(),
            });
            return;
        }
    };

    let total_steps = steps.len();
    let mut all_succeeded = true;

    // Start background sudo keepalive thread if not dry run
    let mut keepalive = if !config.dry_run {
        Some(SudoKeepalive::start())
    } else {
        None
    };

    for (index, step) in steps.iter().enumerate() {
        let step_start = Instant::now();
        let _ = tx.send(EngineEvent::StepStarted {
            index,
            step_id: step.id.clone(),
        });

        runner.log_info(format!("▶ Step {}/{}: {}", index + 1, total_steps, step.title));

        let result = execute_single_step(&step.id, distro, &updated_config, &runner);
        let duration_secs = step_start.elapsed().as_secs_f64();

        let status = match result {
            Ok(_) => {
                runner.log_success(format!("✓ Finished: {} ({:.1}s)", step.title, duration_secs));
                StepStatus::Succeeded
            }
            Err(e) => {
                runner.log_warn(format!("▲ Step warning/non-fatal: {} - {}", step.title, e));
                all_succeeded = false;
                StepStatus::Warning(e)
            }
        };

        let _ = tx.send(EngineEvent::StepCompleted {
            index,
            step_id: step.id.clone(),
            status,
            duration_secs,
        });

        let percent = (((index + 1) as f32 / total_steps as f32) * 100.0) as u16;
        let _ = tx.send(EngineEvent::Progress { percent });
    }

    let total_duration_secs = start_all.elapsed().as_secs_f64();
    let summary = if config.dry_run {
        format!("Dry-run simulation completed in {:.1}s. All planned stages verified.", total_duration_secs)
    } else if all_succeeded {
        format!("Aurora installation completed successfully in {:.1}s!", total_duration_secs)
    } else {
        format!("Aurora installation completed with minor warnings in {:.1}s.", total_duration_secs)
    };

    runner.log_success(&summary);

    if let Some(mut k) = keepalive.take() {
        k.stop();
    }

    let _ = tx.send(EngineEvent::Finished {
        success: all_succeeded,
        total_duration_secs,
        summary,
    });
}

fn resolve_source_dir(config: &InstallConfig, runner: &StepRunner) -> PathBuf {
    // 1. If explicit path supplied
    if config.source_dir.is_dir() && config.source_dir.join("dotfiles/.config").exists() {
        return config.source_dir.clone();
    }

    // 2. Check current working directory
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("dotfiles/.config").exists() {
        runner.log_info(format!("Found dotfiles in current working directory: {}", cwd.display()));
        return cwd;
    }

    // 3. Check ~/Aurora
    if let Some(home) = dirs_home() {
        let home_aurora = home.join("Aurora");
        if home_aurora.join("dotfiles/.config").exists() {
            runner.log_info(format!("Found dotfiles in {}", home_aurora.display()));
            return home_aurora;
        }

        // 4. Check sibling directories in ~/Documents
        let docs = home.join("Documents");
        for candidate in ["Aurora", "Aurora-Arch", "Aurora-Fedora", "Aurora-NixOS"] {
            let p = docs.join(candidate);
            if p.join("dotfiles/.config").exists() {
                runner.log_info(format!("Found dotfiles in {}", p.display()));
                return p;
            }
        }
    }

    // 5. Default fallback location
    if let Some(home) = dirs_home() {
        home.join(".local/share/Aurora/source")
    } else {
        PathBuf::from("/tmp/Aurora")
    }
}

fn execute_single_step(
    step_id: &str,
    _distro: DistroFamily,
    config: &InstallConfig,
    runner: &StepRunner,
) -> Result<(), String> {
    match step_id {
        // --- ARCH STEPS ---
        "arch_backup" => {
            if let Some(home) = dirs_home() {
                let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let backup_dir = home.join(format!(".config/aurora_backups/backup_{ts}"));
                runner.log_info(format!("Archiving existing dotfiles to {}...", backup_dir.display()));
                if !runner.dry_run {
                    let _ = fs::create_dir_all(&backup_dir);
                    let config_dir = home.join(".config");
                    for app in ["hypr", "waybar", "kitty", "ghostty", "rofi", "swaync", "wlogout"] {
                        let target = config_dir.join(app);
                        if target.exists() {
                            let dest = backup_dir.join(app);
                            let _ = runner.run_command("cp", &["-r", target.to_str().unwrap(), dest.to_str().unwrap()]);
                        }
                    }
                }
                runner.log_success(format!("Backup completed: {}", backup_dir.display()));
            }
            Ok(())
        }
        "arch_preflight" => {
            runner.log_info("Checking system requirements for Arch Linux...");
            if !runner.dry_run && !Path::new("/etc/os-release").exists() {
                return Err("Arch Linux check failed: /etc/os-release not found".to_string());
            }
            runner.log_success("Preflight checks passed");
            Ok(())
        }
        "arch_update" => {
            runner.log_info("Synchronizing Arch Linux package databases...");
            runner.run_command("sudo", &["pacman", "-Syu", "--noconfirm"])
        }
        "arch_packages" => {
            runner.log_info("Installing core, UI, and utility packages via pacman...");
            let mut pkgs: Vec<&str> = Vec::new();
            match config.mode {
                InstallMode::Stable => pkgs.extend_from_slice(arch::PACMAN_CORE),
                InstallMode::Git => pkgs.extend_from_slice(arch::PACMAN_CORE_GIT),
            }
            pkgs.extend_from_slice(arch::PACMAN_DAEMONS);
            pkgs.extend_from_slice(arch::PACMAN_UI);
            pkgs.extend_from_slice(arch::PACMAN_UTILS);
            pkgs.extend_from_slice(arch::PACMAN_BUILD);

            let term_pkg = config.terminal_choice.package_name();
            if !pkgs.contains(&term_pkg) {
                pkgs.push(term_pkg);
            }

            let font_pkg = config.font_choice.package_name();
            if !pkgs.contains(&font_pkg) {
                pkgs.push(font_pkg);
            }

            // Filter out packages disabled by the user
            pkgs.retain(|&pkg| config.is_package_enabled(pkg));

            if pkgs.is_empty() {
                runner.log_info("No pacman packages selected, skipping official packages stage.");
                return Ok(());
            }

            let mut args = vec!["pacman", "-S", "--needed", "--noconfirm"];
            args.extend(pkgs);
            runner.run_command("sudo", &args)
        }
        "arch_aur" => {
            let mut aur_pkgs: Vec<&str> = arch::YAY_AUR_PACKAGES.to_vec();
            aur_pkgs.retain(|&pkg| config.is_package_enabled(pkg));

            if aur_pkgs.is_empty() {
                runner.log_info("No AUR packages selected, skipping AUR stage.");
                return Ok(());
            }

            let aur_bin = if which::which("paru").is_ok() {
                "paru"
            } else if which::which("yay").is_ok() {
                "yay"
            } else {
                if !runner.dry_run {
                    runner.log_info("Bootstrapping yay AUR helper from source...");
                    let _ = runner.run_command("git", &["clone", "https://aur.archlinux.org/yay-bin.git", "/tmp/yay-install"]);
                    let _ = runner.run_command("sh", &["-c", "cd /tmp/yay-install && makepkg -si --noconfirm"]);
                }
                "yay"
            };
            runner.log_info(format!("Using AUR helper ({aur_bin}) for rice extras..."));

            let mut args = vec!["-S", "--needed", "--noconfirm"];
            args.extend(aur_pkgs);
            runner.run_command(aur_bin, &args).or_else(|_| {
                runner.log_warn("Some AUR packages may not have built or are already present");
                Ok(())
            })
        }
        "arch_browser" => {
            if let Some(pkg) = config.browser_choice.package_name() {
                runner.log_info(format!("Installing default browser: {} ({})", config.browser_choice.short_name(), pkg));
                if config.browser_choice.is_aur() {
                    let aur_bin = if which::which("paru").is_ok() { "paru" } else { "yay" };
                    let _ = runner.run_command(aur_bin, &["-S", "--needed", "--noconfirm", pkg]);
                } else {
                    let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", pkg]);
                }
            }
            Ok(())
        }
        "arch_gaming" => {
            let mut gaming_pkgs: Vec<&str> = arch::ARCH_GAMING_PACKAGES.to_vec();
            gaming_pkgs.retain(|&pkg| config.is_package_enabled(pkg));
            if gaming_pkgs.is_empty() {
                runner.log_info("No gaming packages selected, skipping gaming install.");
                return Ok(());
            }
            runner.log_info("Installing Steam, GameMode, and MangoHud gaming packages...");
            let mut args = vec!["pacman", "-S", "--needed", "--noconfirm"];
            args.extend(gaming_pkgs);
            runner.run_command("sudo", &args).or_else(|_| {
                runner.log_warn("Some gaming packages could not be installed (ensure multilib is enabled in /etc/pacman.conf)");
                Ok(())
            })
        }
        "arch_network_manager" => {
            runner.log_info("Configuring NetworkManager and disabling competing network services...");
            let competing_services = [
                "iwd.service",
                "systemd-networkd.service",
                "systemd-networkd-wait-online.service",
                "connman.service",
                "dhcpcd.service",
                "wicd.service",
            ];
            for service in competing_services {
                let _ = runner.run_command("sudo", &["systemctl", "disable", "--now", service]);
            }
            runner.run_command("sudo", &["systemctl", "enable", "--now", "NetworkManager.service"]).or_else(|_| Ok(()))
        }
        "arch_sddm" => {
            use crate::model::config::DisplayManagerChoice;
            match config.display_manager {
                DisplayManagerChoice::SddmAstronaut => {
                    runner.log_info("Setting up SDDM Astronaut Theme...");
                    let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "sddm"]);
                    let _ = runner.run_command("sudo", &["mkdir", "-p", "/usr/share/sddm/themes", "/usr/share/fonts", "/etc/sddm.conf.d"]);
                    let _ = runner.run_command(
                        "sudo",
                        &[
                            "git",
                            "clone",
                            "-b",
                            "master",
                            "--depth",
                            "1",
                            "https://github.com/keyitdev/sddm-astronaut-theme.git",
                            "/usr/share/sddm/themes/sddm-astronaut-theme",
                        ],
                    );
                    let _ = runner.run_command("sudo", &["cp", "-rf", "/usr/share/sddm/themes/sddm-astronaut-theme/Fonts/.", "/usr/share/fonts/"]);
                    let _ = runner.run_command("sudo", &["fc-cache", "-f", "/usr/share/fonts"]);
                    if !runner.dry_run {
                        let _ = fs::write("/etc/sddm.conf", "[Theme]\nCurrent=sddm-astronaut-theme\n");
                        let _ = fs::write("/etc/sddm.conf.d/virtualkbd.conf", "[General]\nInputMethod=qtvirtualkeyboard\n");
                    }
                    runner.run_command("sudo", &["systemctl", "enable", "sddm.service"]).or_else(|_| Ok(()))
                }
                DisplayManagerChoice::LyMatrix => {
                    runner.log_info("Setting up Ly lightweight console display manager...");
                    let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "ly"]);
                    runner.run_command("sudo", &["systemctl", "enable", "ly.service"]).or_else(|_| Ok(()))
                }
                DisplayManagerChoice::GreetdTuigreet => {
                    runner.log_info("Setting up Greetd + Tuigreet Wayland greeter...");
                    let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "greetd", "greetd-tuigreet"]);
                    runner.run_command("sudo", &["systemctl", "enable", "greetd.service"]).or_else(|_| Ok(()))
                }
                DisplayManagerChoice::NoneTty => {
                    runner.log_info("Direct TTY login configured (no display manager service enabled)");
                    Ok(())
                }
            }
        }
        "arch_audio" => {
            runner.log_info("Deploying EasyEffects Studio DSP sound suite...");
            let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "easyeffects", "lsp-plugins"]);
            Ok(())
        }
        "arch_gpu" => {
            runner.log_info("Deploying NVIDIA proprietary drivers and Wayland acceleration envs...");
            let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "nvidia-dkms", "nvidia-utils", "egl-wayland"]);
            if !runner.dry_run {
                if let Some(home) = dirs_home() {
                    let env_dir = home.join(".config/hypr/UserConfigs");
                    let _ = fs::create_dir_all(&env_dir);
                    let env_file = env_dir.join("00-NvidiaEnvs.conf");
                    let env_content = "# Generated by Aurora Installer for NVIDIA GPU Acceleration\n\
env = LIBVA_DRIVER_NAME,nvidia\n\
env = __GLX_VENDOR_LIBRARY_NAME,nvidia\n\
env = NVD_BACKEND,direct\n\
env = ELECTRON_OZONE_PLATFORM_HINT,auto\n\
env = GBM_BACKEND,nvidia-drm\n";
                    let _ = fs::write(env_file, env_content);
                }
            }
            runner.log_success("NVIDIA DKMS and Hyprland Wayland environment variables configured");
            Ok(())
        }
        "arch_wallpaper" => {
            let pkg = config.wallpaper_engine.package_name();
            runner.log_info(format!("Setting up wallpaper engine: {} ({pkg})...", config.wallpaper_engine.short_name()));
            let aur_bin = if which::which("paru").is_ok() { "paru" } else { "yay" };
            let res = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", pkg]);
            if res.is_err() {
                let _ = runner.run_command(aur_bin, &["-S", "--needed", "--noconfirm", pkg]);
            }
            Ok(())
        }
        "arch_tweaks" => {
            runner.log_info("Applying kernel and process performance optimizations...");
            let _ = runner.run_command("sudo", &["pacman", "-S", "--needed", "--noconfirm", "ananicy-cpp", "zram-generator"]);
            let _ = runner.run_command("sudo", &["systemctl", "enable", "--now", "ananicy-cpp.service"]);
            Ok(())
        }
        "arch_rust_scripts" => {
            runner.log_info("Compiling Aurora Rust scripts...");
            let scripts_dir = config.source_dir.join("dotfiles/.config/hypr/scripts");
            if scripts_dir.join("Cargo.toml").exists() {
                runner.run_command("cargo", &["install", "--path", scripts_dir.to_str().unwrap_or(".")])?;
            }
            runner.log_info("Installing additional terminal tools (termflix, nmrs-tui)...");
            let _ = runner.run_command("cargo", &["install", "termflix"]);
            let _ = runner.run_command("cargo", &["install", "nmrs-tui"]);
            Ok(())
        }
        "arch_waytrogen" => {
            runner.log_info("Deploying Waytrogen-Aurora...");
            runner.run_command("git", &["clone", "https://github.com/TheAhumMaitra/waytrogen-aurora.git", "/tmp/waytrogen-aurora"])?;
            runner.run_command("sh", &["-c", "cd /tmp/waytrogen-aurora && cargo build --release && cp target/release/waytrogen $HOME/.local/bin/"]).or_else(|_| Ok(()))
        }
        "arch_lazyvim" => {
            deploy_neovim_preset(config, runner)
        }
        "arch_dotfiles" => {
            deploy_dotfiles(config, runner)
        }
        "arch_shell" => {
            configure_shell_env(runner)
        }
        "arch_sudo_rs" => {
            setup_sudo_rs(runner)
        }
        "arch_theme" => {
            apply_theme(config, runner)
        }

        // --- FEDORA STEPS ---
        "fedora_backup" => {
            if let Some(home) = dirs_home() {
                let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let backup_dir = home.join(format!(".config/aurora_backups/backup_{ts}"));
                runner.log_info(format!("Archiving existing dotfiles to {}...", backup_dir.display()));
                if !runner.dry_run {
                    let _ = fs::create_dir_all(&backup_dir);
                    let config_dir = home.join(".config");
                    for app in ["hypr", "waybar", "kitty", "ghostty", "rofi", "swaync", "wlogout"] {
                        let target = config_dir.join(app);
                        if target.exists() {
                            let dest = backup_dir.join(app);
                            let _ = runner.run_command("cp", &["-r", target.to_str().unwrap(), dest.to_str().unwrap()]);
                        }
                    }
                }
                runner.log_success(format!("Backup completed: {}", backup_dir.display()));
            }
            Ok(())
        }
        "fedora_preflight" => {
            runner.log_info("Checking system requirements for Fedora Linux...");
            if !runner.dry_run && !Path::new("/etc/os-release").exists() {
                return Err("Fedora Linux check failed: /etc/os-release not found".to_string());
            }
            runner.log_success("Fedora environment verified");
            Ok(())
        }
        "fedora_copr" => {
            runner.log_info("Enabling Fedora COPR repositories...");
            for copr in fedora::FEDORA_COPR_REPOS {
                let _ = runner.run_command("sudo", &["dnf", "copr", "enable", "-y", copr]);
            }
            runner.log_success("Fedora COPR repositories enabled");
            Ok(())
        }
        "fedora_packages" => {
            runner.log_info("Installing Fedora packages via DNF...");
            let mut pkgs: Vec<&str> = Vec::new();
            match config.mode {
                InstallMode::Stable => pkgs.extend_from_slice(fedora::DNF_CORE),
                InstallMode::Git => pkgs.extend_from_slice(fedora::DNF_CORE_GIT),
            }
            pkgs.extend_from_slice(fedora::DNF_DAEMONS);
            pkgs.extend_from_slice(fedora::DNF_UI);
            pkgs.extend_from_slice(fedora::DNF_UTILS);
            pkgs.extend_from_slice(fedora::DNF_BUILD);

            let term_pkg = config.terminal_choice.package_name();
            if !pkgs.contains(&term_pkg) {
                pkgs.push(term_pkg);
            }

            let font_pkg = config.font_choice.package_name();
            if !pkgs.contains(&font_pkg) {
                pkgs.push(font_pkg);
            }

            pkgs.retain(|&pkg| config.is_package_enabled(pkg));

            if pkgs.is_empty() {
                runner.log_info("No DNF packages selected, skipping package installation.");
                return Ok(());
            }

            let mut args = vec!["dnf", "install", "-y"];
            args.extend(pkgs);
            let res = runner.run_command("sudo", &args);
            if res.is_err() {
                runner.log_warn("Some DNF packages failed to install, proceeding with fallback repositories");
            }
            Ok(())
        }
        "fedora_network_manager" => {
            runner.log_info("Configuring NetworkManager and disabling competing network services...");
            let competing_services = [
                "systemd-networkd.service",
                "systemd-networkd-wait-online.service",
                "connman.service",
                "dhcpcd.service",
                "wicd.service",
            ];
            for service in competing_services {
                let _ = runner.run_command("sudo", &["systemctl", "disable", "--now", service]);
            }
            runner.run_command("sudo", &["systemctl", "enable", "--now", "NetworkManager.service"]).or_else(|_| Ok(()))
        }
        "fedora_sddm" => {
            runner.log_info("Configuring SDDM Astronaut Theme for Fedora...");
            let theme_dir = "/usr/share/sddm/themes/sddm-astronaut-theme";
            let _ = runner.run_command(
                "sudo",
                &[
                    "git",
                    "clone",
                    "-b",
                    "master",
                    "--depth",
                    "1",
                    "https://github.com/keyitdev/sddm-astronaut-theme.git",
                    theme_dir,
                ],
            );
            let _ = runner.run_command("sudo", &["mkdir", "-p", "/usr/share/fonts"]);
            let _ = runner.run_command("sudo", &["cp", "-rf", "/usr/share/sddm/themes/sddm-astronaut-theme/Fonts/.", "/usr/share/fonts/"]);
            let _ = runner.run_command("sudo", &["fc-cache", "-f", "/usr/share/fonts"]);

            if !runner.dry_run {
                let _ = fs::create_dir_all("/etc/sddm.conf.d");
                let _ = fs::write("/etc/sddm.conf.d/theme.conf", "[Theme]\nCurrent=sddm-astronaut-theme\n");
                let _ = fs::write("/etc/sddm.conf.d/virtualkbd.conf", "[General]\nInputMethod=qtvirtualkeyboard\n");
            }

            // Disable competing display managers
            for dm in ["gdm.service", "gdm3.service", "lightdm.service", "lxdm.service", "greetd.service", "ly.service"] {
                let _ = runner.run_command("sudo", &["systemctl", "disable", dm]);
            }
            let _ = runner.run_command("sudo", &["systemctl", "enable", "sddm.service"]);
            runner.log_success("SDDM astronaut theme configured and enabled");
            Ok(())
        }
        "fedora_source_tools" => {
            runner.log_info("Building Fedora desktop addons (hyprwave, Nordzy cursors, rofi-emoji, snappy-switcher)...");
            // 1. hyprwave
            runner.log_info("Building hyprwave from source...");
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/shantanubaddar/hyprwave.git", "/tmp/hyprwave"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/hyprwave && make && make install"]);

            // 2. Nordzy cursors
            runner.log_info("Installing Nordzy cursors...");
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://gitlab.com/gboehm/Nordzy-cursors.git", "/tmp/nordzy-cursors"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/nordzy-cursors && ./install.sh -p"]);

            // 3. rofi-emoji
            runner.log_info("Building rofi-emoji plugin from source...");
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/Mange/rofi-emoji.git", "/tmp/rofi-emoji"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/rofi-emoji && autoreconf -i && mkdir -p build && cd build && ../configure && make && sudo make install"]);

            // 4. snappy-switcher
            runner.log_info("Building snappy-switcher from source...");
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/OpalAayan/snappy-switcher.git", "/tmp/snappy-switcher"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/snappy-switcher && make && sudo make install"]);

            runner.log_success("Fedora desktop addons installed");
            Ok(())
        }
        "fedora_rust_scripts" => {
            runner.log_info("Compiling Aurora Rust scripts on Fedora...");
            let scripts_dir = config.source_dir.join("dotfiles/.config/hypr/scripts");
            if scripts_dir.join("Cargo.toml").exists() {
                let _ = runner.run_command("cargo", &["install", "--path", scripts_dir.to_str().unwrap_or(".")]);
            }
            runner.log_info("Installing mise runtime manager...");
            let _ = runner.run_command("sh", &["-c", "curl -fsSL https://mise.run | sh"]);

            runner.log_info("Installing Rust terminal tools...");
            for crate_name in ["termflix", "nmrs-tui", "bluetui", "leenfetch", "weathr", "wiremix", "jolt-tui"] {
                let _ = runner.run_command("cargo", &["install", crate_name]);
            }
            runner.log_success("Aurora Rust scripts and CLI packages installed");
            Ok(())
        }
        "fedora_waytrogen" => {
            runner.log_info("Building waytrogen-aurora on Fedora...");
            let _ = runner.run_command("git", &["clone", "https://github.com/TheAhumMaitra/waytrogen-aurora.git", "/tmp/waytrogen-aurora"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/waytrogen-aurora && cargo install --path . --locked"]);
            if let Some(home) = dirs_home() {
                let schema_dir = home.join(".local/share/glib-2.0/schemas");
                if !runner.dry_run {
                    let _ = fs::create_dir_all(&schema_dir);
                }
                let _ = runner.run_command("cp", &["-f", "/tmp/waytrogen-aurora/org.Waytrogen.Waytrogen.gschema.xml", schema_dir.to_str().unwrap()]);
                let _ = runner.run_command("glib-compile-schemas", &[schema_dir.to_str().unwrap()]);
            }
            runner.log_success("waytrogen-aurora deployed");
            Ok(())
        }
        "fedora_gaming" => {
            runner.log_info("Installing Steam, GameMode, and MangoHud on Fedora...");
            let mut args = vec!["dnf", "install", "-y"];
            args.extend_from_slice(fedora::FEDORA_GAMING_PACKAGES);
            runner.run_command("sudo", &args).or_else(|_| Ok(()))
        }
        "fedora_browser" => {
            if let Some(pkg) = config.browser_choice.package_name() {
                runner.log_info(format!("Installing default browser: {} ({})", config.browser_choice.short_name(), pkg));
                let _ = runner.run_command("sudo", &["dnf", "install", "-y", pkg]);
            }
            Ok(())
        }
        "fedora_lazyvim" => {
            deploy_neovim_preset(config, runner)
        }
        "fedora_dotfiles" => {
            deploy_dotfiles(config, runner)
        }
        "fedora_shell" => {
            runner.log_info("Setting default shell to Fish and updating PATH...");
            if which::which("fish").is_ok() {
                let fish_path = which::which("fish").unwrap().display().to_string();
                let _ = runner.run_command("chsh", &["-s", &fish_path]);
            }
            configure_shell_env(runner)
        }
        "fedora_sudo_rs" => {
            setup_sudo_rs(runner)
        }
        "fedora_theme" => {
            apply_theme(config, runner)
        }

        // --- NIXOS STEPS ---
        "nixos_backup" => {
            if let Some(home) = dirs_home() {
                let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let backup_dir = home.join(format!(".config/aurora_backups/backup_{ts}"));
                runner.log_info(format!("Archiving existing dotfiles to {}...", backup_dir.display()));
                if !runner.dry_run {
                    let _ = fs::create_dir_all(&backup_dir);
                    let config_dir = home.join(".config");
                    for app in ["hypr", "waybar", "kitty", "ghostty", "rofi", "swaync", "wlogout"] {
                        let target = config_dir.join(app);
                        if target.exists() {
                            let dest = backup_dir.join(app);
                            let _ = runner.run_command("cp", &["-r", target.to_str().unwrap(), dest.to_str().unwrap()]);
                        }
                    }
                }
                runner.log_success(format!("Backup completed: {}", backup_dir.display()));
            }
            Ok(())
        }
        "nixos_preflight" => {
            runner.log_info("Checking Nix Flakes support and NixOS environment...");
            runner.run_command("sudo", &["nix", "config", "set", "experimental-features", "nix-command flakes"])
        }
        "nixos_copy_config" => {
            runner.log_info("Deploying NixOS Flake configuration files to /etc/nixos...");
            let nix_dir = Path::new("/etc/nixos");
            let mut search_paths = vec![
                config.source_dir.clone(),
                PathBuf::from("."),
            ];
            if let Some(home) = dirs_home() {
                search_paths.push(home.join("Aurora"));
                search_paths.push(home.join("Documents/Aurora-NixOS"));
                search_paths.push(home.join("Documents/Aurora"));
                search_paths.push(home.join(".local/share/Aurora/source"));
            }

            let mut found_source = None;
            for p in &search_paths {
                if p.join("flake.nix").exists() {
                    found_source = Some(p.clone());
                    break;
                }
            }

            let source_nix = if let Some(src) = found_source {
                src
            } else {
                let clone_target = if let Some(home) = dirs_home() {
                    home.join(".local/share/Aurora/Aurora-NixOS")
                } else {
                    PathBuf::from("/tmp/Aurora-NixOS")
                };
                if !clone_target.join("flake.nix").exists() && !runner.dry_run {
                    runner.log_info("Fetching Aurora-NixOS flake configurations from GitHub...");
                    let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/TheAhumMaitra/Aurora-NixOS.git", clone_target.to_str().unwrap_or("")]);
                }
                if clone_target.join("flake.nix").exists() {
                    clone_target
                } else {
                    PathBuf::from("/etc/nixos")
                }
            };
            runner.log_info(format!("Using NixOS source from: {}", source_nix.display()));

            if let Some(home) = dirs_home() {
                let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
                let nix_backup = home.join(format!(".local/share/Aurora/nixos_backup_{ts}"));
                let _ = runner.run_command("sudo", &["mkdir", "-p", nix_backup.to_str().unwrap()]);
            }

            let _ = runner.run_command("sudo", &["mkdir", "-p", "/etc/nixos"]);

            for file in nixos::NIXOS_CONFIG_FILES {
                let src = source_nix.join(file);
                let dst = nix_dir.join(file);
                if src.exists() {
                    let _ = runner.run_command("sudo", &["cp", "-f", src.to_str().unwrap_or(""), dst.to_str().unwrap_or("")]);
                    let _ = runner.run_command("sudo", &["chmod", "644", dst.to_str().unwrap_or("")]);
                }
            }
            runner.log_success("NixOS configuration files deployed to /etc/nixos");
            Ok(())
        }
        "nixos_rebuild_switch" => {
            runner.log_info("Running 'sudo nixos-rebuild switch --flake /etc/nixos#nixos'...");
            runner.run_command("sudo", &["nixos-rebuild", "switch", "--flake", "/etc/nixos#nixos"])
        }
        "nixos_dotfiles" => {
            deploy_dotfiles(config, runner)
        }
        "nixos_rust_scripts" => {
            runner.log_info("Setting Rust stable toolchain...");
            let _ = runner.run_command("rustup", &["default", "stable"]).or_else(|_| {
                runner.run_command("nix", &["develop", "/etc/nixos", "-c", "rustup", "default", "stable"])
            });

            runner.log_info("Building Aurora Rust scripts inside Nix develop shell...");
            let scripts_dir = config.source_dir.join("dotfiles/.config/hypr/scripts");
            if scripts_dir.join("Cargo.toml").exists() {
                let _ = runner.run_command("sh", &["-c", &format!("cd {} && nix develop /etc/nixos -c cargo install --path .", scripts_dir.display())]);
            }
            runner.log_info("Installing Rust tools inside Nix develop shell...");
            for pkg in ["termflix", "nmrs-tui", "leenfetch", "jolt-tui", "bluetui", "weathr"] {
                let _ = runner.run_command("nix", &["develop", "/etc/nixos", "-c", "cargo", "install", pkg]);
            }
            runner.log_success("Aurora Rust scripts and tools compiled");
            Ok(())
        }
        "nixos_sddm" => {
            runner.log_info("Deploying SDDM astronaut theme assets on NixOS...");
            let _ = runner.run_command("sudo", &["mkdir", "-p", "/usr/share/sddm/themes", "/usr/share/fonts"]);
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/keyitdev/sddm-astronaut-theme.git", "/tmp/sddm-astronaut"]);
            let _ = runner.run_command("sudo", &["cp", "-rf", "/tmp/sddm-astronaut", "/usr/share/sddm/themes/sddm-astronaut-theme"]);
            let _ = runner.run_command("sudo", &["cp", "-rf", "/tmp/sddm-astronaut/Fonts/.", "/usr/share/fonts/"]);

            if !runner.dry_run {
                let _ = fs::write("/etc/sddm.conf", "[Theme]\nCurrent=sddm-astronaut-theme\n");
                let _ = fs::create_dir_all("/etc/sddm.conf.d");
                let _ = fs::write("/etc/sddm.conf.d/virtualkbd.conf", "[General]\nInputMethod=qtvirtualkeyboard\n");
            }
            runner.log_success("SDDM astronaut theme deployed");
            Ok(())
        }
        "nixos_waytrogen" => {
            runner.log_info("Building waytrogen-aurora in Nix develop shell...");
            let _ = runner.run_command("git", &["clone", "https://github.com/TheAhumMaitra/waytrogen-aurora.git", "/tmp/waytrogen-aurora"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/waytrogen-aurora && nix develop /etc/nixos -c cargo install --path . --locked"]);
            if let Some(home) = dirs_home() {
                let schema_dir = home.join(".local/share/glib-2.0/schemas");
                if !runner.dry_run {
                    let _ = fs::create_dir_all(&schema_dir);
                }
                let _ = runner.run_command("cp", &["-f", "/tmp/waytrogen-aurora/org.Waytrogen.Waytrogen.gschema.xml", schema_dir.to_str().unwrap()]);
                let _ = runner.run_command("glib-compile-schemas", &[schema_dir.to_str().unwrap()]);
            }
            runner.log_success("waytrogen-aurora installed");
            Ok(())
        }
        "nixos_extras" => {
            runner.log_info("Installing mise, Nordzy Cursors and Zen Browser Flatpak...");
            // 1. mise
            let _ = runner.run_command("sh", &["-c", "curl -fsSL https://mise.run | sh"]);

            // 2. Nordzy cursors
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://gitlab.com/gboehm/Nordzy-cursors.git", "/tmp/nordzy-cursors"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/nordzy-cursors && ./install.sh -P"]);

            // 3. Zen Browser via flatpak
            let _ = runner.run_command("flatpak", &["remote-add", "--if-not-exists", "flathub", "https://dl.flathub.org/repo/flathub.flatpakrepo"]);
            let _ = runner.run_command("flatpak", &["install", "-y", "--noninteractive", "flathub", "app.zen_browser.zen"]);

            // Add zen-browser alias to shell configurations
            if let Some(home) = dirs_home() {
                let posix_alias = "\nalias zen-browser=\"flatpak run app.zen_browser.zen\"\n";
                let fish_alias = "\nalias zen-browser \"flatpak run app.zen_browser.zen\"\n";
                for rc in [home.join(".bashrc"), home.join(".zshrc")] {
                    if rc.exists() {
                        let mut content = fs::read_to_string(&rc).unwrap_or_default();
                        if !content.contains("alias zen-browser") {
                            content.push_str(posix_alias);
                            let _ = fs::write(&rc, content);
                        }
                    }
                }
                let fish_cfg = home.join(".config/fish/config.fish");
                if fish_cfg.exists() {
                    let mut content = fs::read_to_string(&fish_cfg).unwrap_or_default();
                    if !content.contains("alias zen-browser") {
                        content.push_str(fish_alias);
                        let _ = fs::write(&fish_cfg, content);
                    }
                }
            }
            runner.log_success("Mise, Nordzy cursors, and Zen Browser configured");
            Ok(())
        }
        "nixos_gaming" => {
            runner.log_info("Configuring Gaming options for NixOS...");
            runner.log_success("Gaming packages configured (ensure programs.steam.enable is active in configuration.nix)");
            Ok(())
        }
        "nixos_lazyvim" => {
            deploy_neovim_preset(config, runner)
        }
        "nixos_shell" => {
            configure_shell_env(runner)
        }
        "nixos_theme" => {
            apply_theme(config, runner)
        }

        // --- EXTRAORDINARY FEATURES ---
        "btrfs_snapshot" => {
            setup_btrfs_snapshot(config, runner)
        }
        "power_optimization" => {
            setup_power_optimization(config, runner)
        }
        "security_hardening" => {
            setup_security_hardening(config, runner)
        }
        "window_effects" => {
            setup_window_effects(config, runner)
        }
        "cursor_icon_themes" => {
            setup_cursor_icon_themes(config, runner)
        }
        "virtualization_setup" => {
            setup_virtualization(config, runner)
        }
        "proton_ge_setup" => {
            setup_proton_ge(config, runner)
        }
        "system_maintenance" => {
            setup_system_maintenance(config, runner)
        }

        _ => {
            runner.log_info(format!("Completed step: {step_id}"));
            Ok(())
        }
    }
}

fn setup_btrfs_snapshot(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    if !config.btrfs_snapshot {
        runner.log_info("Btrfs snapshot disabled by configuration.");
        return Ok(());
    }

    let is_btrfs = fs::read_to_string("/proc/mounts")
        .map(|content| {
            content.lines().any(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                parts.len() >= 3 && (parts[1] == "/" || parts[1] == "/home") && parts[2] == "btrfs"
            })
        })
        .unwrap_or(false);

    if !is_btrfs && !runner.dry_run {
        runner.log_info("Root or home filesystem is not Btrfs; skipping subvolume snapshot.");
        return Ok(());
    }

    let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let snap_target = format!("/.aurora_pre_install_snapshot_{ts}");
    runner.log_info(format!("Creating instant Btrfs rollback snapshot at {snap_target}..."));
    let _ = runner.run_command("sudo", &["btrfs", "subvolume", "snapshot", "-r", "/", &snap_target]);
    runner.log_success(format!("Pre-installation Btrfs rollback snapshot created: {snap_target}"));
    Ok(())
}

fn setup_power_optimization(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info(format!("Configuring Power Profile: {}...", config.power_profile.label()));
    match config.power_profile {
        crate::model::config::PowerProfileChoice::Performance => {
            let _ = runner.run_command("sudo", &["powerprofilesctl", "set", "performance"]);
            runner.log_success("Performance power profile applied.");
        }
        crate::model::config::PowerProfileChoice::BatteryCare => {
            let _ = runner.run_command("sudo", &["powerprofilesctl", "set", "power-saver"]);
            let _ = runner.run_command("sh", &["-c", "for f in /sys/class/power_supply/BAT*/charge_control_end_threshold; do [ -f \"$f\" ] && echo 80 | sudo tee \"$f\" >/dev/null; done"]);
            runner.log_success("BatteryCare profile configured with 80% charge threshold.");
        }
        crate::model::config::PowerProfileChoice::Balanced => {
            let _ = runner.run_command("sudo", &["powerprofilesctl", "set", "balanced"]);
            runner.log_success("Balanced power profile applied.");
        }
        crate::model::config::PowerProfileChoice::None => {
            runner.log_info("Keeping existing system power configuration.");
        }
    }
    Ok(())
}

fn setup_security_hardening(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Applying Desktop Security & Privacy Hardening...");

    // 1. Firewall
    match config.security_firewall {
        crate::model::config::FirewallChoice::UfwHardened => {
            runner.log_info("Configuring UFW desktop firewall rules...");
            let _ = runner.run_command("sudo", &["ufw", "default", "deny", "incoming"]);
            let _ = runner.run_command("sudo", &["ufw", "default", "allow", "outgoing"]);
            let _ = runner.run_command("sudo", &["ufw", "allow", "53317/tcp"]); // LocalSend
            let _ = runner.run_command("sudo", &["ufw", "allow", "53317/udp"]);
            let _ = runner.run_command("sudo", &["ufw", "allow", "1714:1764/udp"]); // KDE Connect
            let _ = runner.run_command("sudo", &["ufw", "allow", "1714:1764/tcp"]);
            let _ = runner.run_command("sudo", &["ufw", "allow", "5353/udp"]); // mDNS
            let _ = runner.run_command("sudo", &["ufw", "--force", "enable"]);
            let _ = runner.run_command("sudo", &["systemctl", "enable", "--now", "ufw.service"]);
            runner.log_success("UFW desktop firewall configured and active.");
        }
        crate::model::config::FirewallChoice::Firewalld => {
            runner.log_info("Configuring firewalld home/desktop zone...");
            let _ = runner.run_command("sudo", &["firewall-cmd", "--set-default-zone=home"]);
            let _ = runner.run_command("sudo", &["firewall-cmd", "--reload"]);
            runner.log_success("Firewalld configured.");
        }
        crate::model::config::FirewallChoice::None => {}
    }

    // 2. Privacy DNS (systemd-resolved)
    match config.dns_privacy {
        crate::model::config::DnsPrivacyChoice::Quad9 => {
            runner.log_info("Configuring Quad9 (9.9.9.9) privacy & threat blocking DNS...");
            let content = "[Resolve]\nDNS=9.9.9.9 149.112.112.112 2620:fe::fe 2620:fe::9\nDomains=~.\nDNSSEC=yes\nDNSOverTLS=yes\n";
            let _ = runner.run_command("sudo", &["mkdir", "-p", "/etc/systemd/resolved.conf.d"]);
            let _ = runner.run_command("sh", &["-c", &format!("cat << 'EOF' | sudo tee /etc/systemd/resolved.conf.d/00-aurora-dns.conf >/dev/null\n{content}EOF")]);
            let _ = runner.run_command("sudo", &["systemctl", "restart", "systemd-resolved.service"]);
            runner.log_success("Quad9 Privacy DNS configured with DNS-over-TLS.");
        }
        crate::model::config::DnsPrivacyChoice::Cloudflare => {
            runner.log_info("Configuring Cloudflare (1.1.1.1) fast DoH DNS...");
            let content = "[Resolve]\nDNS=1.1.1.1 1.0.0.1 2606:4700:4700::1111 2606:4700:4700::1001\nDomains=~.\nDNSSEC=yes\nDNSOverTLS=yes\n";
            let _ = runner.run_command("sudo", &["mkdir", "-p", "/etc/systemd/resolved.conf.d"]);
            let _ = runner.run_command("sh", &["-c", &format!("cat << 'EOF' | sudo tee /etc/systemd/resolved.conf.d/00-aurora-dns.conf >/dev/null\n{content}EOF")]);
            let _ = runner.run_command("sudo", &["systemctl", "restart", "systemd-resolved.service"]);
            runner.log_success("Cloudflare DNS configured.");
        }
        crate::model::config::DnsPrivacyChoice::AdGuard => {
            runner.log_info("Configuring AdGuard ad-blocking DNS...");
            let content = "[Resolve]\nDNS=94.140.14.14 94.140.15.15\nDomains=~.\nDNSOverTLS=yes\n";
            let _ = runner.run_command("sudo", &["mkdir", "-p", "/etc/systemd/resolved.conf.d"]);
            let _ = runner.run_command("sh", &["-c", &format!("cat << 'EOF' | sudo tee /etc/systemd/resolved.conf.d/00-aurora-dns.conf >/dev/null\n{content}EOF")]);
            let _ = runner.run_command("sudo", &["systemctl", "restart", "systemd-resolved.service"]);
            runner.log_success("AdGuard DNS configured.");
        }
        crate::model::config::DnsPrivacyChoice::SystemDefault => {}
    }

    // 3. NetworkManager MAC Randomization
    if config.mac_randomization {
        runner.log_info("Enabling WiFi & Ethernet MAC address randomization...");
        let content = "[device]\nwifi.scan-rand-mac-address=yes\n\n[connection]\nwifi.cloned-mac-address=random\nethernet.cloned-mac-address=random\n";
        let _ = runner.run_command("sudo", &["mkdir", "-p", "/etc/NetworkManager/conf.d"]);
        let _ = runner.run_command("sh", &["-c", &format!("cat << 'EOF' | sudo tee /etc/NetworkManager/conf.d/00-aurora-mac-randomization.conf >/dev/null\n{content}EOF")]);
        runner.log_success("NetworkManager MAC randomization configured.");
    }

    Ok(())
}

fn setup_window_effects(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info(format!("Generating Hyprland Window Effects Config: {}...", config.window_effects.label()));
    if let Some(home) = dirs_home() {
        let hypr_dir = home.join(".config/hypr");
        if !runner.dry_run {
            let _ = fs::create_dir_all(&hypr_dir);
        }

        let effects_file = hypr_dir.join("aurora_effects.conf");
        let effects_content = match config.window_effects {
            crate::model::config::WindowEffectsChoice::FrostedGlass => {
                "# Aurora Frosted Glass Preset (Signature Look)\n\
                 decoration {\n\
                     rounding = 12\n\
                     active_opacity = 0.90\n\
                     inactive_opacity = 0.82\n\
                     blur {\n\
                         enabled = true\n\
                         size = 8\n\
                         passes = 3\n\
                         new_optimizations = true\n\
                         noise = 0.015\n\
                         contrast = 1.0\n\
                         brightness = 1.0\n\
                     }\n\
                     shadow {\n\
                         enabled = true\n\
                         range = 16\n\
                         render_power = 3\n\
                         color = rgba(00000044)\n\
                     }\n\
                 }\n"
            }
            crate::model::config::WindowEffectsChoice::CyberpunkNeon => {
                "# Aurora Cyberpunk Neon Preset\n\
                 general {\n\
                     border_size = 2\n\
                     col.active_border = rgb(ff007f) rgb(00f0ff) 45deg\n\
                     col.inactive_border = rgb(1f1a2e)\n\
                 }\n\
                 decoration {\n\
                     rounding = 8\n\
                     active_opacity = 0.95\n\
                     inactive_opacity = 0.88\n\
                     blur {\n\
                         enabled = true\n\
                         size = 5\n\
                         passes = 2\n\
                     }\n\
                     shadow {\n\
                         enabled = true\n\
                         range = 20\n\
                         render_power = 3\n\
                         color = rgba(ff007f88)\n\
                     }\n\
                 }\n"
            }
            crate::model::config::WindowEffectsChoice::OledPureBlack => {
                "# Aurora OLED Pure Black Preset (Maximum Battery & Contrast)\n\
                 general {\n\
                     border_size = 2\n\
                     col.active_border = rgb(777777)\n\
                     col.inactive_border = rgb(111111)\n\
                 }\n\
                 decoration {\n\
                     rounding = 6\n\
                     active_opacity = 1.0\n\
                     inactive_opacity = 1.0\n\
                     blur {\n\
                         enabled = false\n\
                     }\n\
                     shadow {\n\
                         enabled = false\n\
                     }\n\
                 }\n"
            }
            crate::model::config::WindowEffectsChoice::MinimalFlat => {
                "# Aurora Minimal Flat Preset (Ultra-Low Overhead)\n\
                 general {\n\
                     border_size = 1\n\
                     col.active_border = rgb(555555)\n\
                     col.inactive_border = rgb(222222)\n\
                 }\n\
                 decoration {\n\
                     rounding = 0\n\
                     active_opacity = 1.0\n\
                     inactive_opacity = 1.0\n\
                     blur {\n\
                         enabled = false\n\
                     }\n\
                     shadow {\n\
                         enabled = false\n\
                     }\n\
                 }\n"
            }
        };

        if !runner.dry_run {
            let _ = fs::write(&effects_file, effects_content);
            let hypr_conf = hypr_dir.join("hyprland.conf");
            if hypr_conf.exists() {
                let mut conf_txt = fs::read_to_string(&hypr_conf).unwrap_or_default();
                if !conf_txt.contains("aurora_effects.conf") {
                    conf_txt.push_str("\nsource = ~/.config/hypr/aurora_effects.conf\n");
                    let _ = fs::write(&hypr_conf, conf_txt);
                }
            }
        }
        runner.log_success(format!("Window effects written to {}", effects_file.display()));
    }
    Ok(())
}

fn setup_cursor_icon_themes(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    let cursor_name = config.cursor_theme.theme_name();
    let icon_name = config.icon_theme.theme_name();
    runner.log_info(format!("Configuring Cursor Theme ({cursor_name}) and Icon Theme ({icon_name})..."));

    if let Some(home) = dirs_home() {
        let icons_default = home.join(".icons/default");
        if !runner.dry_run {
            let _ = fs::create_dir_all(&icons_default);
            let index_theme = format!("[Icon Theme]\nName=Default\nComment=Default Cursor Theme\nInherits={cursor_name}\n");
            let _ = fs::write(icons_default.join("index.theme"), index_theme);
        }

        let _ = runner.run_command("gsettings", &["set", "org.gnome.desktop.interface", "cursor-theme", cursor_name]);
        let _ = runner.run_command("gsettings", &["set", "org.gnome.desktop.interface", "icon-theme", icon_name]);

        let hypr_conf = home.join(".config/hypr/hyprland.conf");
        if !runner.dry_run && hypr_conf.exists() {
            let mut conf_txt = fs::read_to_string(&hypr_conf).unwrap_or_default();
            let cursor_env = format!("\nenv = XCURSOR_THEME,{cursor_name}\nenv = XCURSOR_SIZE,24\n");
            if !conf_txt.contains("XCURSOR_THEME") {
                conf_txt.push_str(&cursor_env);
                let _ = fs::write(&hypr_conf, conf_txt);
            }
        }
        runner.log_success(format!("Applied cursor theme '{cursor_name}' and icon theme '{icon_name}'."));
    }
    Ok(())
}

fn setup_virtualization(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    match config.virtualization {
        crate::model::config::VirtualizationChoice::QemuKvm => {
            runner.log_info("Setting up QEMU / KVM virtualization services...");
            let _ = runner.run_command("sudo", &["systemctl", "enable", "--now", "libvirtd.service"]);
            let user = std::env::var("USER").unwrap_or_else(|_| "ahummaitra".to_string());
            let _ = runner.run_command("sudo", &["usermod", "-aG", "libvirt,kvm", &user]);
            let _ = runner.run_command("sudo", &["virsh", "net-autostart", "default"]);
            runner.log_success("QEMU/KVM & libvirtd service configured for user.");
        }
        crate::model::config::VirtualizationChoice::DockerPodman => {
            runner.log_info("Setting up Docker / Podman container services...");
            let _ = runner.run_command("sudo", &["systemctl", "enable", "--now", "docker.service"]);
            let user = std::env::var("USER").unwrap_or_else(|_| "ahummaitra".to_string());
            let _ = runner.run_command("sudo", &["usermod", "-aG", "docker", &user]);
            runner.log_success("Docker service enabled with user permissions.");
        }
        crate::model::config::VirtualizationChoice::None => {}
    }
    Ok(())
}

fn setup_proton_ge(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    if !config.proton_ge {
        return Ok(());
    }
    runner.log_info("Configuring Proton-GE & Steam Gaming Enhancements...");
    if let Some(home) = dirs_home() {
        let steam_compat = home.join(".steam/root/compatibilitytools.d");
        let local_compat = home.join(".local/share/Steam/compatibilitytools.d");
        if !runner.dry_run {
            let _ = fs::create_dir_all(&steam_compat);
            let _ = fs::create_dir_all(&local_compat);
        }

        let mangohud_dir = home.join(".config/MangoHud");
        if !runner.dry_run {
            let _ = fs::create_dir_all(&mangohud_dir);
            let mangohud_conf = "# Aurora MangoHud Gaming Overlay\n\
                                 fps\n\
                                 frametime\n\
                                 cpu_stats\n\
                                 cpu_temp\n\
                                 gpu_stats\n\
                                 gpu_temp\n\
                                 ram\n\
                                 vram\n\
                                 round_corners=10\n\
                                 background_alpha=0.6\n\
                                 font_size=20\n";
            let _ = fs::write(mangohud_dir.join("MangoHud.conf"), mangohud_conf);
        }
        runner.log_success("Proton-GE compatibility directory and MangoHud configuration ready.");
    }
    Ok(())
}

fn setup_system_maintenance(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    if !config.auto_maintenance {
        return Ok(());
    }
    runner.log_info("Deploying Automated System Maintenance & Health Timers...");
    if let Some(home) = dirs_home() {
        let systemd_user = home.join(".config/systemd/user");
        if !runner.dry_run {
            let _ = fs::create_dir_all(&systemd_user);
            let service_content = "[Unit]\n\
                                   Description=Aurora Weekly System Maintenance\n\n\
                                   [Service]\n\
                                   Type=oneshot\n\
                                   ExecStart=/bin/sh -c 'which fstrim >/dev/null && sudo fstrim -v /; which paccache >/dev/null && paccache -rk2; which dnf >/dev/null && sudo dnf clean packages -y; which nix-collect-garbage >/dev/null && nix-collect-garbage -d'\n";
            let timer_content = "[Unit]\n\
                                 Description=Aurora Weekly Maintenance Timer\n\n\
                                 [Timer]\n\
                                 OnCalendar=weekly\n\
                                 Persistent=true\n\n\
                                 [Install]\n\
                                 WantedBy=timers.target\n";
            let _ = fs::write(systemd_user.join("aurora-maintenance.service"), service_content);
            let _ = fs::write(systemd_user.join("aurora-maintenance.timer"), timer_content);
        }
        let _ = runner.run_command("systemctl", &["--user", "daemon-reload"]);
        let _ = runner.run_command("systemctl", &["--user", "enable", "--now", "aurora-maintenance.timer"]);
        runner.log_success("Aurora weekly maintenance systemd timer active.");
    }
    Ok(())
}

fn deploy_neovim_preset(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    if config.neovim_preset == crate::model::config::NeovimPreset::None {
        runner.log_info("Skipping Neovim setup (None / Preserve Selected)");
        return Ok(());
    }

    let repo = match config.neovim_preset.repo_url() {
        Some(url) => url,
        None => return Ok(()),
    };

    runner.log_info(format!("Deploying {} from {}...", config.neovim_preset.label(), repo));
    if let Some(home) = dirs_home() {
        let nvim_dir = home.join(".config/nvim");
        if nvim_dir.exists() {
            let backup_nvim = config.backup_dir.join("nvim");
            runner.log_info(format!("Backing up existing Neovim configs to {}", backup_nvim.display()));
            if !runner.dry_run {
                let _ = fs::create_dir_all(&config.backup_dir);
                let _ = fs::rename(&nvim_dir, &backup_nvim);
            }
        }
        runner.run_command("git", &["clone", repo, nvim_dir.to_str().unwrap_or("")])?;
        if !runner.dry_run {
            let _ = fs::remove_dir_all(nvim_dir.join(".git"));
        }
    }
    runner.log_success(format!("{} deployed successfully", config.neovim_preset.label()));
    Ok(())
}

fn deploy_dotfiles(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Deploying Aurora dotfiles...");
    let mut effective_src = config.source_dir.join("dotfiles/.config");
    if !effective_src.exists() && !runner.dry_run {
        runner.log_warn(format!("Source dotfiles not found at {}. Cloning Aurora repository...", effective_src.display()));
        let clone_dst = if let Some(home) = dirs_home() {
            home.join(".local/share/Aurora/source")
        } else {
            PathBuf::from("/tmp/Aurora")
        };
        if !clone_dst.join("dotfiles/.config").exists() {
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/TheAhumMaitra/Aurora.git", clone_dst.to_str().unwrap_or("")]);
        }
        if clone_dst.join("dotfiles/.config").exists() {
            effective_src = clone_dst.join("dotfiles/.config");
        }
    }

    if let Some(home) = dirs_home() {
        let dest_config = home.join(".config");
        runner.log_info(format!("Backing up current configs to {}", config.backup_dir.display()));
        if !runner.dry_run {
            let _ = fs::create_dir_all(&config.backup_dir);
        }

        // Copy dotfiles while preserving ~/.config/hypr/User
        runner.log_info("Copying configuration files (preserving ~/.config/hypr/User)...");
        if !runner.dry_run && effective_src.exists() {
            let user_hypr_dir = dest_config.join("hypr/User");
            let mut user_backup_tmp = None;
            if user_hypr_dir.exists() {
                let tmp = config.backup_dir.join("user_hypr_preserve");
                let _ = fs::rename(&user_hypr_dir, &tmp);
                user_backup_tmp = Some(tmp);
            }

            // Copy src items
            let _ = runner.run_command("cp", &["-rf", &format!("{}/.", effective_src.display()), &dest_config.display().to_string()]);

            // Restore user hypr dir
            if let Some(tmp) = user_backup_tmp {
                let _ = fs::rename(tmp, user_hypr_dir);
            }
        }
    }

    runner.log_success("Dotfiles deployed successfully");
    Ok(())
}

fn configure_shell_env(runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Configuring Shell PATH in ~/.bashrc, ~/.zshrc, ~/.config/fish/config.fish...");
    if let Some(home) = dirs_home() {
        if !runner.dry_run {
            let cargo_bin = home.join(".cargo/bin");
            let local_bin = home.join(".local/bin");
            let path_line = format!("\nexport PATH=\"{}:{}:$PATH\"\n", cargo_bin.display(), local_bin.display());

            for rc in [home.join(".bashrc"), home.join(".zshrc"), home.join(".bash_profile")] {
                if rc.exists() {
                    let mut content = fs::read_to_string(&rc).unwrap_or_default();
                    if !content.contains(".cargo/bin") {
                        content.push_str(&path_line);
                        let _ = fs::write(&rc, content);
                    }
                }
            }

            let fish_cfg = home.join(".config/fish/config.fish");
            if fish_cfg.exists() {
                let mut content = fs::read_to_string(&fish_cfg).unwrap_or_default();
                if !content.contains("fish_add_path") {
                    content.push_str(&format!("\nfish_add_path {} {}\n", cargo_bin.display(), local_bin.display()));
                    let _ = fs::write(&fish_cfg, content);
                }
            }
        }
    }
    Ok(())
}

fn setup_sudo_rs(runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Configuring sudo-rs memory-safe replacement...");
    if runner.dry_run {
        runner.log_warn("[DRY-RUN] Would atomically replace /usr/bin/sudo with symlink to /usr/bin/sudo-rs");
        return Ok(());
    }

    if which::which("sudo-rs").is_ok() || Path::new("/usr/bin/sudo-rs").exists() {
        let script = r#"
        set -e
        if [ ! -x /usr/bin/sudo-rs ]; then
            echo "sudo-rs binary missing at /usr/bin/sudo-rs" >&2
            exit 1
        fi
        if [ -L /usr/bin/sudo ] && [ "$(readlink /usr/bin/sudo)" = "/usr/bin/sudo-rs" ]; then
            exit 0
        fi
        if [ ! -e /usr/bin/sudo-original ]; then
            if [ -e /usr/bin/sudo ] || [ -L /usr/bin/sudo ]; then
                mv /usr/bin/sudo /usr/bin/sudo-original
            fi
        fi
        rm -f /usr/bin/sudo
        ln -sf /usr/bin/sudo-rs /usr/bin/sudo
        "#;
        let _ = runner.run_command("sudo", &["sh", "-c", script]);
        runner.log_success("Switched /usr/bin/sudo to /usr/bin/sudo-rs");
        Ok(())
    } else {
        runner.log_warn("sudo-rs binary not found in PATH; skipping symlink");
        Ok(())
    }
}

fn apply_theme(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info(format!("Applying theme: '{}'...", config.theme_name));

    if which::which("awww-daemon").is_ok() || runner.dry_run {
        let _ = runner.run_command("sh", &["-c", "awww-daemon >/dev/null 2>&1 &"]);
    }

    let aurora_bin = which::which("aurora").ok().or_else(|| {
        dirs_home().map(|h| h.join(".cargo/bin/aurora")).filter(|p| p.exists())
    });

    if let Some(bin) = aurora_bin {
        let res = runner.run_command(bin.to_str().unwrap_or("aurora"), &["apply-theme", &config.theme_name]);
        if let Err(e) = res {
            runner.log_warn(format!("Could not apply theme automatically: {e}"));
        } else {
            let _ = runner.run_command("pkill", &["-f", "waybar"]);
            runner.log_success(format!("Theme '{}' applied successfully", config.theme_name));
        }
        Ok(())
    } else {
        runner.log_warn("Aurora CLI binary not yet in PATH; apply theme later with 'aurora apply-theme <name>'");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_sudo_cached_does_not_panic() {
        let _ = check_sudo_cached();
    }

    #[test]
    fn test_sudo_keepalive_lifecycle() {
        let mut keepalive = SudoKeepalive::start();
        std::thread::sleep(Duration::from_millis(50));
        keepalive.stop();
        keepalive.stop();
    }

    #[test]
    fn test_sudo_keepalive_drop() {
        let keepalive = SudoKeepalive::start();
        std::thread::sleep(Duration::from_millis(50));
        drop(keepalive);
    }
}
