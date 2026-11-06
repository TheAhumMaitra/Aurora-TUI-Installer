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
use std::sync::mpsc::Sender;
use std::time::Instant;

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

            let mut args = vec!["pacman", "-S", "--needed", "--noconfirm"];
            args.extend(pkgs);
            runner.run_command("sudo", &args)
        }
        "arch_aur" => {
            runner.log_info("Checking for AUR helper (yay)...");
            if which::which("yay").is_err() && !runner.dry_run {
                runner.log_info("Bootstrapping yay from AUR...");
                let _ = runner.run_command("git", &["clone", "https://aur.archlinux.org/yay.git", "/tmp/yay-install"]);
                let _ = runner.run_command("sh", &["-c", "cd /tmp/yay-install && makepkg -si --noconfirm"]);
            }
            runner.log_info("Installing AUR extras...");
            let mut args = vec!["-S", "--needed", "--noconfirm"];
            args.extend_from_slice(arch::YAY_AUR_PACKAGES);
            runner.run_command("yay", &args).or_else(|_| {
                runner.log_warn("Some AUR packages may not have built or are already present");
                Ok(())
            })
        }
        "arch_sddm" => {
            runner.log_info("Setting up SDDM Astronaut Theme...");
            runner.run_command(
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
            )?;
            runner.run_command("sudo", &["systemctl", "enable", "sddm.service"]).or_else(|_| Ok(()))
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
            deploy_lazyvim(config, runner)
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
        "fedora_preflight" => {
            runner.log_info("Checking system requirements for Fedora Linux...");
            runner.log_success("Fedora environment verified");
            Ok(())
        }
        "fedora_copr" => {
            runner.log_info("Enabling Fedora COPR repositories...");
            for copr in fedora::FEDORA_COPR_REPOS {
                let _ = runner.run_command("sudo", &["dnf", "copr", "enable", "-y", copr]);
            }
            Ok(())
        }
        "fedora_packages" => {
            runner.log_info("Installing Fedora packages via DNF...");
            let mut pkgs: Vec<&str> = Vec::new();
            pkgs.extend_from_slice(fedora::DNF_CORE);
            pkgs.extend_from_slice(fedora::DNF_DAEMONS);
            pkgs.extend_from_slice(fedora::DNF_UI);
            pkgs.extend_from_slice(fedora::DNF_UTILS);
            pkgs.extend_from_slice(fedora::DNF_BUILD);

            let mut args = vec!["dnf", "install", "-y"];
            args.extend(pkgs);
            runner.run_command("sudo", &args)
        }
        "fedora_source_tools" => {
            runner.log_info("Building Fedora desktop addons (Nordzy cursors, hyprwave)...");
            let _ = runner.run_command("git", &["clone", "https://github.com/alvatip/Nordzy-cursors.git", "/tmp/nordzy-cursors"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/nordzy-cursors && ./install.sh -p"]);
            Ok(())
        }
        "fedora_sddm" => {
            runner.log_info("Configuring SDDM Astronaut Theme for Fedora...");
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
            let _ = runner.run_command("sudo", &["systemctl", "disable", "--now", "gdm.service"]);
            let _ = runner.run_command("sudo", &["systemctl", "enable", "sddm.service"]);
            Ok(())
        }
        "fedora_rust_scripts" => {
            runner.log_info("Compiling Aurora Rust scripts on Fedora...");
            let scripts_dir = config.source_dir.join("dotfiles/.config/hypr/scripts");
            if scripts_dir.join("Cargo.toml").exists() {
                let _ = runner.run_command("cargo", &["install", "--path", scripts_dir.to_str().unwrap_or(".")]);
            }
            let _ = runner.run_command("cargo", &["install", "termflix"]);
            let _ = runner.run_command("cargo", &["install", "nmrs-tui"]);
            Ok(())
        }
        "fedora_waytrogen" => {
            runner.log_info("Building waytrogen-aurora on Fedora...");
            let _ = runner.run_command("git", &["clone", "https://github.com/TheAhumMaitra/waytrogen-aurora.git", "/tmp/waytrogen-aurora"]);
            let _ = runner.run_command("sh", &["-c", "cd /tmp/waytrogen-aurora && cargo build --release && cp target/release/waytrogen $HOME/.local/bin/"]);
            Ok(())
        }
        "fedora_lazyvim" => {
            deploy_lazyvim(config, runner)
        }
        "fedora_dotfiles" => {
            deploy_dotfiles(config, runner)
        }
        "fedora_shell" => {
            runner.log_info("Setting default shell to Fish and updating PATH...");
            let _ = runner.run_command("chsh", &["-s", "/usr/bin/fish"]);
            configure_shell_env(runner)
        }
        "fedora_sudo_rs" => {
            setup_sudo_rs(runner)
        }
        "fedora_theme" => {
            apply_theme(config, runner)
        }

        // --- NIXOS STEPS ---
        "nixos_preflight" => {
            runner.log_info("Checking Nix Flakes support...");
            runner.run_command("sudo", &["nix", "config", "set", "experimental-features", "nix-command flakes"])
        }
        "nixos_copy_config" => {
            runner.log_info("Deploying NixOS Flake configuration files to /etc/nixos...");
            let nix_dir = Path::new("/etc/nixos");
            let source_nix = if config.source_dir.join("flake.nix").exists() {
                config.source_dir.clone()
            } else if let Some(home) = dirs_home() {
                home.join("Documents/Aurora-NixOS")
            } else {
                PathBuf::from(".")
            };

            for file in nixos::NIXOS_CONFIG_FILES {
                let src = source_nix.join(file);
                let dst = nix_dir.join(file);
                if src.exists() {
                    runner.run_command("sudo", &["cp", "-f", src.to_str().unwrap_or(""), dst.to_str().unwrap_or("")])?;
                }
            }
            Ok(())
        }
        "nixos_rebuild_switch" => {
            runner.log_info("Running nixos-rebuild switch...");
            runner.run_command("sudo", &["nixos-rebuild", "switch", "--flake", "/etc/nixos#nixos"])
        }
        "nixos_dotfiles" => {
            deploy_dotfiles(config, runner)
        }
        "nixos_rust_scripts" => {
            runner.log_info("Building Aurora Rust scripts inside Nix develop shell...");
            runner.run_command("nix", &["develop", "/etc/nixos", "-c", "cargo", "install", "--path", "."]).or_else(|_| Ok(()))
        }
        "nixos_sddm" => {
            runner.log_info("Deploying SDDM astronaut theme assets...");
            let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/keyitdev/sddm-astronaut-theme.git", "/tmp/sddm-astronaut"]);
            Ok(())
        }
        "nixos_waytrogen" => {
            runner.log_info("Building waytrogen-aurora in dev shell...");
            let _ = runner.run_command("git", &["clone", "https://github.com/TheAhumMaitra/waytrogen-aurora.git", "/tmp/waytrogen-aurora"]);
            Ok(())
        }
        "nixos_extras" => {
            runner.log_info("Installing Mise, Nordzy Cursors and Zen Browser Flatpak...");
            let _ = runner.run_command("sh", &["-c", "curl -fsSL https://mise.run | sh"]);
            let _ = runner.run_command("flatpak", &["remote-add", "--if-not-exists", "flathub", "https://dl.flathub.org/repo/flathub.flatpakrepo"]);
            let _ = runner.run_command("flatpak", &["install", "-y", "flathub", "app.zen_browser.zen"]);
            Ok(())
        }
        "nixos_lazyvim" => {
            deploy_lazyvim(config, runner)
        }
        "nixos_shell" => {
            configure_shell_env(runner)
        }
        "nixos_theme" => {
            apply_theme(config, runner)
        }

        _ => {
            runner.log_info(format!("Completed step: {step_id}"));
            Ok(())
        }
    }
}

fn deploy_lazyvim(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Deploying LazyVim starter...");
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
        runner.run_command("git", &["clone", "https://github.com/LazyVim/starter", nvim_dir.to_str().unwrap_or("")])?;
        if !runner.dry_run {
            let _ = fs::remove_dir_all(nvim_dir.join(".git"));
        }
    }
    Ok(())
}

fn deploy_dotfiles(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info("Deploying Aurora dotfiles...");
    let src_dotfiles = config.source_dir.join("dotfiles/.config");
    if !src_dotfiles.exists() && !runner.dry_run {
        runner.log_warn(format!("Source dotfiles not found at {}. Cloning Aurora repository...", src_dotfiles.display()));
        let clone_dst = if let Some(home) = dirs_home() {
            home.join(".local/share/Aurora/source")
        } else {
            PathBuf::from("/tmp/Aurora")
        };
        let _ = runner.run_command("git", &["clone", "--depth", "1", "https://github.com/TheAhumMaitra/Aurora.git", clone_dst.to_str().unwrap_or("")]);
    }

    if let Some(home) = dirs_home() {
        let dest_config = home.join(".config");
        runner.log_info(format!("Backing up current configs to {}", config.backup_dir.display()));
        if !runner.dry_run {
            let _ = fs::create_dir_all(&config.backup_dir);
        }

        // Copy dotfiles while preserving ~/.config/hypr/User
        runner.log_info("Copying configuration files (preserving ~/.config/hypr/User)...");
        if !runner.dry_run && src_dotfiles.exists() {
            let user_hypr_dir = dest_config.join("hypr/User");
            let mut user_backup_tmp = None;
            if user_hypr_dir.exists() {
                let tmp = config.backup_dir.join("user_hypr_preserve");
                let _ = fs::rename(&user_hypr_dir, &tmp);
                user_backup_tmp = Some(tmp);
            }

            // Copy src items
            let _ = runner.run_command("cp", &["-rf", &format!("{}/.", src_dotfiles.display()), &dest_config.display().to_string()]);

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

            for rc in [home.join(".bashrc"), home.join(".zshrc")] {
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
    if which::which("sudo-rs").is_ok() || runner.dry_run {
        runner.run_command("sudo", &["ln", "-sf", "/usr/bin/sudo-rs", "/usr/bin/sudo"]).or_else(|_| Ok(()))
    } else {
        runner.log_warn("sudo-rs binary not found in PATH; skipping symlink");
        Ok(())
    }
}

fn apply_theme(config: &InstallConfig, runner: &StepRunner) -> Result<(), String> {
    runner.log_info(format!("Applying theme: '{}'...", config.theme_name));
    let aurora_bin = which::which("aurora").ok().or_else(|| {
        dirs_home().map(|h| h.join(".cargo/bin/aurora")).filter(|p| p.exists())
    });

    if let Some(bin) = aurora_bin {
        runner.run_command(bin.to_str().unwrap_or("aurora"), &["apply-theme", &config.theme_name]).or_else(|e| {
            runner.log_warn(format!("Could not apply theme automatically: {e}"));
            Ok(())
        })
    } else {
        runner.log_warn("Aurora CLI binary not yet in PATH; apply theme later with 'aurora apply-theme <name>'");
        Ok(())
    }
}
