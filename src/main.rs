// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod distro;
mod engine;
mod model;
mod ui;

use clap::Parser;
use cli::{
    AudioArg, BackupArg, BarLayoutArg, BrowserArg, CliArgs, CursorArg, DisplayManagerArg,
    DistroArg, DnsArg, FirewallArg, FontArg, GpuArg, IconArg, LauncherArg, LockscreenArg, ModeArg,
    NeovimArg, PowerArg, PresetArg, ShellArg, TerminalArg, TweaksArg, VirtArg, WallpaperArg,
    WindowEffectsArg,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use distro::DistroInfo;
use model::config::{
    AudioSuiteChoice, BackupStrategy, BarLayoutChoice, BrowserChoice, Component,
    CursorThemeChoice, DisplayManagerChoice, DnsPrivacyChoice, FirewallChoice, FontChoice,
    GpuProfile, IconThemeChoice, InstallConfig, InstallMode, InstallPreset, KernelTweakProfile,
    LauncherStyle, LockscreenChoice, NeovimPreset, PowerProfileChoice, ShellChoice, TerminalChoice,
    VirtualizationChoice, WallpaperEngine, WindowEffectsChoice,
};
use model::state::{AppScreen, AppState, ConfigSection};
use model::theme::AVAILABLE_THEMES;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // System detection
    let distro_info = DistroInfo::detect();

    // Fast-path hardware diagnostics report check
    if args.diagnostics {
        print_diagnostics_report(&distro_info);
        return Ok(());
    }

    // Fast-path System Doctor verification check
    if args.doctor {
        let dummy_cfg = InstallConfig::default();
        let report = engine::run_doctor(&distro_info, &dummy_cfg);
        engine::print_doctor_report(&report);
        return Ok(());
    }

    // Prepare default config or import from profile
    let mut config = if let Some(ref profile_path) = args.profile {
        println!("Loading Aurora installer profile from: {}", profile_path.display());
        InstallConfig::import_from_json(profile_path)?
    } else {
        InstallConfig::default()
    };

    // Override with CLI flags
    if args.dry_run {
        config.dry_run = true;
    }
    if args.no_sudo_rs {
        config.allow_sudo_rs = false;
    }
    if let Some(mode) = args.mode {
        config.mode = match mode {
            ModeArg::Stable => InstallMode::Stable,
            ModeArg::Git => InstallMode::Git,
        };
    }
    if let Some(preset) = args.preset {
        match preset {
            PresetArg::Full => config.apply_preset(InstallPreset::FullRice),
            PresetArg::Minimal => config.apply_preset(InstallPreset::MinimalRice),
            PresetArg::Gaming => config.apply_preset(InstallPreset::GamingRice),
            PresetArg::Oled => config.apply_preset(InstallPreset::OledLaptopRice),
        }
    }
    if let Some(neovim) = args.neovim {
        config.neovim_preset = match neovim {
            NeovimArg::LazyVim => NeovimPreset::LazyVim,
            NeovimArg::NvChad => NeovimPreset::NvChad,
            NeovimArg::AstroNvim => NeovimPreset::AstroNvim,
            NeovimArg::Kickstart => NeovimPreset::Kickstart,
            NeovimArg::None => NeovimPreset::None,
        };
    }
    if let Some(term) = args.terminal {
        config.terminal_choice = match term {
            TerminalArg::Kitty => TerminalChoice::Kitty,
            TerminalArg::Ghostty => TerminalChoice::Ghostty,
            TerminalArg::Alacritty => TerminalChoice::Alacritty,
            TerminalArg::Foot => TerminalChoice::Foot,
        };
    }
    if let Some(sh) = args.shell {
        config.shell_choice = match sh {
            ShellArg::Fish => ShellChoice::Fish,
            ShellArg::Zsh => ShellChoice::Zsh,
            ShellArg::Bash => ShellChoice::Bash,
        };
    }
    if let Some(b) = args.browser {
        config.browser_choice = match b {
            BrowserArg::Zen => BrowserChoice::Zen,
            BrowserArg::Brave => BrowserChoice::Brave,
            BrowserArg::Firefox => BrowserChoice::Firefox,
            BrowserArg::Chromium => BrowserChoice::Chromium,
            BrowserArg::None => BrowserChoice::None,
        };
    }
    if let Some(dm) = args.dm {
        config.display_manager = match dm {
            DisplayManagerArg::Sddm => DisplayManagerChoice::SddmAstronaut,
            DisplayManagerArg::Ly => DisplayManagerChoice::LyMatrix,
            DisplayManagerArg::Greetd => DisplayManagerChoice::GreetdTuigreet,
            DisplayManagerArg::None => DisplayManagerChoice::NoneTty,
        };
    }
    if let Some(bar) = args.bar {
        config.bar_layout = match bar {
            BarLayoutArg::Island => BarLayoutChoice::FloatingIsland,
            BarLayoutArg::Full => BarLayoutChoice::FullWidthEdge,
            BarLayoutArg::Dual => BarLayoutChoice::DualBar,
            BarLayoutArg::Minimal => BarLayoutChoice::MinimalClean,
        };
    }
    if let Some(l) = args.launcher {
        config.launcher_style = match l {
            LauncherArg::Spotlight => LauncherStyle::Spotlight,
            LauncherArg::Drawer => LauncherStyle::FullscreenDrawer,
            LauncherArg::Corner => LauncherStyle::CornerPopup,
        };
    }
    if let Some(a) = args.audio {
        config.audio_suite = match a {
            AudioArg::EasyEffects => AudioSuiteChoice::EasyEffectsStudio,
            AudioArg::Pipewire => AudioSuiteChoice::PurePipewire,
        };
    }
    if let Some(t) = args.tweaks {
        config.kernel_tweaks = match t {
            TweaksArg::Performance => KernelTweakProfile::Performance,
            TweaksArg::Standard => KernelTweakProfile::Standard,
        };
    }
    if let Some(font) = args.font {
        config.font_choice = match font {
            FontArg::JetBrainsMono => FontChoice::JetBrainsMono,
            FontArg::FiraCode => FontChoice::FiraCode,
            FontArg::CascadiaCode => FontChoice::CascadiaCode,
            FontArg::Hack => FontChoice::Hack,
        };
    }
    if let Some(lock) = args.lockscreen {
        config.lockscreen = match lock {
            LockscreenArg::Hyprlock => LockscreenChoice::Hyprlock,
            LockscreenArg::SwaylockEffects => LockscreenChoice::SwaylockEffects,
            LockscreenArg::None => LockscreenChoice::None,
        };
    }
    if let Some(gpu) = args.gpu {
        config.gpu_profile = match gpu {
            GpuArg::Auto => GpuProfile::AutoDetect,
            GpuArg::Nvidia => GpuProfile::NvidiaProprietary,
            GpuArg::Amd => GpuProfile::AmdMesa,
            GpuArg::Intel => GpuProfile::IntelMesa,
            GpuArg::VirtualMachine => GpuProfile::VirtualMachine,
        };
    }
    if let Some(wp) = args.wallpaper {
        config.wallpaper_engine = match wp {
            WallpaperArg::Swww => WallpaperEngine::Swww,
            WallpaperArg::Hyprpaper => WallpaperEngine::Hyprpaper,
            WallpaperArg::Awww => WallpaperEngine::Awww,
            WallpaperArg::MpvPaper => WallpaperEngine::MpvPaper,
        };
    }
    if let Some(bk) = args.backup {
        config.backup_strategy = match bk {
            BackupArg::Timestamped => BackupStrategy::Timestamped,
            BackupArg::DirectOverwrite => BackupStrategy::DirectOverwrite,
            BackupArg::SkipExisting => BackupStrategy::SkipExisting,
        };
    }
    if let Some(ref export_path) = args.export_profile {
        config.export_to_json(export_path)?;
        println!("Successfully exported Aurora installer profile to: {}", export_path.display());
        return Ok(());
    }
    if let Some(excluded) = args.exclude_packages {
        for pkg in excluded {
            config.disable_package(&pkg);
        }
    }
    if args.gaming && !config.selected_components.contains(&Component::GamingSuite) {
        config.selected_components.push(Component::GamingSuite);
    }
    if let Some(eff) = args.effects {
        config.window_effects = match eff {
            WindowEffectsArg::Frosted => WindowEffectsChoice::FrostedGlass,
            WindowEffectsArg::Neon => WindowEffectsChoice::CyberpunkNeon,
            WindowEffectsArg::Oled => WindowEffectsChoice::OledPureBlack,
            WindowEffectsArg::Flat => WindowEffectsChoice::MinimalFlat,
        };
    }
    if let Some(p) = args.power {
        config.power_profile = match p {
            PowerArg::Balanced => PowerProfileChoice::Balanced,
            PowerArg::Performance => PowerProfileChoice::Performance,
            PowerArg::Battery => PowerProfileChoice::BatteryCare,
            PowerArg::None => PowerProfileChoice::None,
        };
        if config.power_profile != PowerProfileChoice::None && !config.selected_components.contains(&Component::PowerOptimization) {
            config.selected_components.push(Component::PowerOptimization);
        }
    }
    if let Some(fw) = args.firewall {
        config.security_firewall = match fw {
            FirewallArg::Ufw => FirewallChoice::UfwHardened,
            FirewallArg::Firewalld => FirewallChoice::Firewalld,
            FirewallArg::None => FirewallChoice::None,
        };
    }
    if let Some(dns) = args.dns {
        config.dns_privacy = match dns {
            DnsArg::Quad9 => DnsPrivacyChoice::Quad9,
            DnsArg::Cloudflare => DnsPrivacyChoice::Cloudflare,
            DnsArg::AdGuard => DnsPrivacyChoice::AdGuard,
            DnsArg::System => DnsPrivacyChoice::SystemDefault,
        };
    }
    if args.mac_randomize {
        config.mac_randomization = true;
    }
    if let Some(c) = args.cursor {
        config.cursor_theme = match c {
            CursorArg::Nordzy => CursorThemeChoice::Nordzy,
            CursorArg::Bibata => CursorThemeChoice::BibataModern,
            CursorArg::Catppuccin => CursorThemeChoice::CatppuccinMocha,
            CursorArg::Breeze => CursorThemeChoice::Breeze,
        };
    }
    if let Some(i) = args.icons {
        config.icon_theme = match i {
            IconArg::Papirus => IconThemeChoice::PapirusDark,
            IconArg::Tela => IconThemeChoice::TelaCircle,
            IconArg::WhiteSur => IconThemeChoice::WhiteSur,
            IconArg::Fluent => IconThemeChoice::FluentDark,
        };
    }
    if let Some(v) = args.virt {
        config.virtualization = match v {
            VirtArg::Qemu => VirtualizationChoice::QemuKvm,
            VirtArg::Docker => VirtualizationChoice::DockerPodman,
            VirtArg::None => VirtualizationChoice::None,
        };
        if config.virtualization != VirtualizationChoice::None && !config.selected_components.contains(&Component::Virtualization) {
            config.selected_components.push(Component::Virtualization);
        }
    }
    if args.no_snapshot {
        config.btrfs_snapshot = false;
    }
    if args.no_maintenance {
        config.auto_maintenance = false;
    }
    if args.proton_ge {
        config.proton_ge = true;
    }
    if let Some(theme) = args.theme {
        config.theme_name = theme;
    }
    if let Some(dir) = args.dir {
        config.source_dir = dir;
    }

    let mut state = AppState::new(distro_info, config);

    // Apply distro override from CLI if specified
    if let Some(distro_arg) = args.distro {
        let family = match distro_arg {
            DistroArg::Arch => distro::DistroFamily::Arch,
            DistroArg::Fedora => distro::DistroFamily::Fedora,
            DistroArg::NixOS => distro::DistroFamily::NixOS,
        };
        state.set_distro(family);
    }

    // Apply explicit package selection if specified
    if let Some(explicit_pkgs) = args.packages {
        let all = state.selected_distro.all_packages();
        state.config.disable_all_global(all);
        state.config.preset = InstallPreset::Custom;
        for pkg in explicit_pkgs {
            state.config.enable_package(&pkg);
        }
        state.refresh_steps();
    }

    // Headless / non-interactive execution
    if args.non_interactive {
        run_headless(&mut state);
        return Ok(());
    }

    // Interactive TUI
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_tui_loop(&mut terminal, &mut state);

    // Teardown terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error during execution: {err:?}");
    }

    Ok(())
}

fn print_diagnostics_report(info: &DistroInfo) {
    println!("================================================================================");
    println!("             AURORA PREFLIGHT HARDWARE & SYSTEM DIAGNOSTICS                     ");
    println!("================================================================================");
    println!("Operating System:    {}", info.distro_flavor);
    println!("Distro Family:       {}", info.family.label());
    println!("Hardware Chassis:    {}", if info.is_laptop { format!("Laptop ({:?}% Battery)", info.battery_percent.unwrap_or(100)) } else { "Desktop Workstation".to_string() });
    println!("Root Filesystem:     {}", if info.is_btrfs { "Btrfs (Snapshots Supported)" } else { "Standard Linux FS" });
    println!("Kernel Version:      {} ({})", info.kernel, info.arch);
    println!("Firmware Mode:       {}", if info.is_uefi { "UEFI Mode (Optimal)" } else { "Legacy BIOS" });
    println!("Processor (CPU):     {} ({} cores)", info.cpu_model, info.cpu_cores);
    println!("Memory (RAM):        {:.1} GB total ({:.1} GB available)", info.total_ram_gb, info.available_ram_gb);
    println!("Storage (/home):     {:.1} GB free space", info.home_free_gb);
    println!("Graphics Hardware:   {}", info.gpu.label());
    println!("Package Manager:     {}", info.family.package_manager());
    if info.family == distro::DistroFamily::Arch {
        println!("AUR Helper:          {}", info.aur_helper.as_deref().unwrap_or("none (will bootstrap yay)"));
    }
    println!("Session User:        {} (root: {})", info.current_user, info.is_root);
    println!("Wayland / Hyprland:  {}", if info.hyprland_running { "Hyprland session active" } else { "Ready for Wayland launch" });
    println!("--------------------------------------------------------------------------------");
    let all_ok = info.is_compatible && info.home_free_gb >= 5.0 && info.total_ram_gb >= 2.0;
    if all_ok {
        println!("PREFLIGHT RESULT:    10/10 PASS - System is fully compatible with Aurora!");
    } else {
        println!("PREFLIGHT RESULT:    MINOR WARNINGS DETECTED - Installation can proceed.");
    }
    println!("================================================================================");
}

fn run_headless(state: &mut AppState) {
    println!("=== Aurora Non-Interactive Installation ===");
    println!("Detected System:     {}", state.distro_info.distro_flavor);
    println!("Target Distro:       {}", state.selected_distro.name());
    println!(
        "Distro Support:      {}",
        if state.distro_info.is_compatible {
            "Verified & Supported"
        } else {
            "Incompatible / Unsupported"
        }
    );
    println!(
        "Package Manager:     {}",
        state.selected_distro.package_manager()
    );
    if state.selected_distro == distro::DistroFamily::Arch {
        println!(
            "AUR Helper:          {}",
            state
                .distro_info
                .aur_helper
                .as_deref()
                .unwrap_or("none (will bootstrap yay)")
        );
    }
    println!("GPU Hardware:        {}", state.distro_info.gpu.label());
    println!("GPU Profile:         {}", state.config.gpu_profile.label());
    println!("Installation Mode:   {:?}", state.config.mode);
    println!("Neovim Preset:       {}", state.config.neovim_preset.label());
    println!("Terminal Choice:     {}", state.config.terminal_choice.label());
    println!("Web Browser:         {}", state.config.browser_choice.label());
    println!("Display Manager:     {}", state.config.display_manager.label());
    println!("Lockscreen Daemon:   {}", state.config.lockscreen.label());
    println!("Wallpaper Engine:    {}", state.config.wallpaper_engine.label());
    println!("Backup Strategy:     {}", state.config.backup_strategy.label());
    println!("Monospace Font:      {}", state.config.font_choice.label());
    println!("Waybar Layout:       {}", state.config.bar_layout.label());
    println!("Rofi Launcher:       {}", state.config.launcher_style.label());
    println!("Audio Suite:         {}", state.config.audio_suite.label());
    println!("Kernel Tweaks:       {}", state.config.kernel_tweaks.label());
    println!("Shell Choice:        {}", state.config.shell_choice.label());
    println!("Window Effects:      {}", state.config.window_effects.label());
    println!("Power Profile:       {}", state.config.power_profile.label());
    println!("Firewall:            {}", state.config.security_firewall.label());
    println!("Privacy DNS:         {}", state.config.dns_privacy.label());
    println!("Cursor Theme:        {}", state.config.cursor_theme.label());
    println!("Icon Theme:          {}", state.config.icon_theme.label());
    println!("Virtualization:      {}", state.config.virtualization.label());
    println!("Btrfs Snapshot:      {}", state.config.btrfs_snapshot);
    println!("Auto Maintenance:    {}", state.config.auto_maintenance);
    println!("Proton-GE Setup:     {}", state.config.proton_ge);
    println!("Theme:               {}", state.config.theme_name);
    let all_pkgs = state.selected_distro.all_packages();
    let enabled_count = all_pkgs.iter().filter(|&&p| state.config.is_package_enabled(p)).count();
    println!("Active Packages:     {}/{} selected", enabled_count, all_pkgs.len());
    println!("Dry Run:             {}", state.config.dry_run);
    println!("===========================================");

    if !state.config.dry_run && !state.distro_info.is_root {
        if !engine::check_sudo_cached() {
            println!("\n[Aurora Installer] Administrative (sudo) privileges required.");
            println!("Please enter your sudo password in the terminal below:\n");
            let status = Command::new("sudo")
                .arg("-v")
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            if !status.map(|s| s.success()).unwrap_or(false) {
                eprintln!("\nError: Sudo authentication failed or was cancelled. Aborting installation.");
                std::process::exit(1);
            }
            println!("\n[✓] Sudo authentication verified successfully.\n");
        }
    }

    state.start_installation();

    while state.is_installing {
        state.poll_events();
        std::thread::sleep(Duration::from_millis(50));
    }

    println!("\nInstallation Summary:");
    println!("{}", state.summary_message);
}

fn run_tui_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut AppState,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        // Poll worker thread updates
        state.poll_events();

        // Draw active frame
        terminal.draw(|f| ui::render_app(f, state))?;

        if state.should_exit {
            break;
        }

        // Handle keyboard & input events
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(state, key.code, key.modifiers, terminal)?;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state.tick_spinner();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn trigger_installation(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut AppState,
) -> io::Result<()> {
    if state.config.dry_run || state.distro_info.is_root || engine::check_sudo_cached() {
        state.sudo_error = None;
        state.is_sudo_cached = true;
        state.start_installation();
        return Ok(());
    }

    // Temporarily exit TUI alternate screen and raw mode for clean terminal password entry
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!();
    println!("\x1b[1;36m╔════════════════════════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║                    AURORA INSTALLER - PRIVILEGE ESCALATION                     ║\x1b[0m");
    println!("\x1b[1;36m╚════════════════════════════════════════════════════════════════════════════════╝\x1b[0m");
    println!();
    println!("\x1b[1mNotice:\x1b[0m Installing system packages, configuring display managers, and");
    println!("        enabling systemd services require administrative (sudo) privileges.");
    println!();
    println!("Please enter your sudo password in the terminal below:");
    println!();

    let status = Command::new("sudo")
        .arg("-v")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let auth_ok = status.map(|s| s.success()).unwrap_or(false);

    if auth_ok {
        println!();
        println!("\x1b[1;32m[✓] Sudo authentication verified successfully. Resuming installer...\x1b[0m");
        std::thread::sleep(Duration::from_millis(600));
    } else {
        println!();
        println!("\x1b[1;31m[✗] Sudo authentication failed or was cancelled.\x1b[0m");
        std::thread::sleep(Duration::from_millis(1200));
    }

    // Re-enter TUI alternate screen & raw mode
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;

    if auth_ok {
        state.sudo_error = None;
        state.is_sudo_cached = true;
        state.start_installation();
    } else {
        state.sudo_error = Some(
            "Sudo authentication failed or was cancelled. Press [Enter] or [Y] to retry."
                .to_string(),
        );
    }

    Ok(())
}

fn handle_key_event(
    state: &mut AppState,
    code: KeyCode,
    modifiers: KeyModifiers,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> io::Result<()> {
    // Global exit shortcut: Ctrl+C
    if modifiers.contains(KeyModifiers::CONTROL)
        && (code == KeyCode::Char('c') || code == KeyCode::Char('C'))
    {
        state.should_exit = true;
        return Ok(());
    }

    // Global 'q' / 'Q' exit shortcut at any time (except when actively typing in package search)
    if (code == KeyCode::Char('q') || code == KeyCode::Char('Q'))
        && !(state.current_screen == AppScreen::PackageSelection && state.package_search_active)
    {
        state.should_exit = true;
        return Ok(());
    }

    match state.current_screen {
        AppScreen::Welcome => match code {
            KeyCode::Enter => {
                state.current_screen = AppScreen::Configuration;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                state.current_screen = AppScreen::Diagnostics;
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                state.cycle_distro();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                state.should_exit = true;
            }
            _ => {}
        },

        AppScreen::Configuration => match code {
            KeyCode::Tab => {
                state.active_config_section = match state.active_config_section {
                    ConfigSection::Preset => ConfigSection::Developer,
                    ConfigSection::Developer => ConfigSection::Aesthetics,
                    ConfigSection::Aesthetics => ConfigSection::Theme,
                    ConfigSection::Theme => ConfigSection::SystemFlags,
                    ConfigSection::SystemFlags => ConfigSection::Components,
                    ConfigSection::Components => ConfigSection::Preset,
                };
            }
            KeyCode::BackTab => {
                state.active_config_section = match state.active_config_section {
                    ConfigSection::Preset => ConfigSection::Components,
                    ConfigSection::Developer => ConfigSection::Preset,
                    ConfigSection::Aesthetics => ConfigSection::Developer,
                    ConfigSection::Theme => ConfigSection::Aesthetics,
                    ConfigSection::SystemFlags => ConfigSection::Theme,
                    ConfigSection::Components => ConfigSection::SystemFlags,
                };
            }
            // Extraordinary shortcuts!
            KeyCode::Char('k') | KeyCode::Char('K') => {
                state.package_category_cursor = 0;
                state.package_item_cursor = 0;
                state.package_side_packages = false;
                state.current_screen = AppScreen::PackageSelection;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                state.config.enable_all_global();
                state.refresh_steps();
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                let all = state.selected_distro.all_packages();
                state.config.disable_all_global(all);
                state.refresh_steps();
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let next = state.config.preset.cycle();
                state.config.apply_preset(next);
                state.refresh_steps();
            }
            KeyCode::Char('1') => {
                state.config.power_profile = state.config.power_profile.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('2') => {
                state.config.window_effects = state.config.window_effects.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('3') => {
                state.config.cursor_theme = state.config.cursor_theme.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('4') => {
                state.config.icon_theme = state.config.icon_theme.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('5') => {
                state.config.security_firewall = state.config.security_firewall.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('6') => {
                state.config.dns_privacy = state.config.dns_privacy.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('7') => {
                state.config.virtualization = state.config.virtualization.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('8') => {
                state.config.btrfs_snapshot = !state.config.btrfs_snapshot;
                state.refresh_steps();
            }
            KeyCode::Char('9') => {
                state.config.auto_maintenance = !state.config.auto_maintenance;
                state.refresh_steps();
            }
            KeyCode::Char('0') => {
                state.config.mac_randomization = !state.config.mac_randomization;
                state.refresh_steps();
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                state.config.neovim_preset = state.config.neovim_preset.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                state.config.terminal_choice = state.config.terminal_choice.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                state.config.browser_choice = state.config.browser_choice.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                state.config.bar_layout = state.config.bar_layout.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                state.config.launcher_style = state.config.launcher_style.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                state.config.display_manager = state.config.display_manager.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                state.config.lockscreen = state.config.lockscreen.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                state.config.font_choice = state.config.font_choice.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('j') | KeyCode::Char('J') => {
                state.config.wallpaper_engine = state.config.wallpaper_engine.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                state.config.gpu_profile = state.config.gpu_profile.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                state.config.backup_strategy = state.config.backup_strategy.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                state.current_screen = AppScreen::Diagnostics;
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                state.config.audio_suite = state.config.audio_suite.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                state.config.kernel_tweaks = state.config.kernel_tweaks.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                state.config.shell_choice = state.config.shell_choice.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                state.config.animation_profile = state.config.animation_profile.cycle();
                state.refresh_steps();
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                state.config.mode = match state.config.mode {
                    InstallMode::Stable => InstallMode::Git,
                    InstallMode::Git => InstallMode::Stable,
                };
                state.refresh_steps();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                state.config.toggle_component(Component::GamingSuite);
                state.refresh_steps();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                state.config.dry_run = !state.config.dry_run;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                state.config.allow_sudo_rs = !state.config.allow_sudo_rs;
                state.refresh_steps();
            }
            KeyCode::Up => match state.active_config_section {
                ConfigSection::Components => {
                    state.component_cursor = state.component_cursor.saturating_sub(1);
                }
                ConfigSection::Theme => {
                    state.theme_cursor = state.theme_cursor.saturating_sub(1);
                    state.config.theme_name =
                        AVAILABLE_THEMES[state.theme_cursor].name.to_string();
                    state.refresh_steps();
                }
                _ => {}
            },
            KeyCode::Down => match state.active_config_section {
                ConfigSection::Components => {
                    if state.component_cursor + 1 < Component::ALL.len() {
                        state.component_cursor += 1;
                    }
                }
                ConfigSection::Theme => {
                    if state.theme_cursor + 1 < AVAILABLE_THEMES.len() {
                        state.theme_cursor += 1;
                        state.config.theme_name =
                            AVAILABLE_THEMES[state.theme_cursor].name.to_string();
                        state.refresh_steps();
                    }
                }
                _ => {}
            },
            KeyCode::Char(' ') => match state.active_config_section {
                ConfigSection::Components => {
                    let comp = Component::ALL[state.component_cursor];
                    state.config.toggle_component(comp);
                    state.refresh_steps();
                }
                ConfigSection::SystemFlags => {
                    state.config.dry_run = !state.config.dry_run;
                }
                ConfigSection::Preset => {
                    let next = state.config.preset.cycle();
                    state.config.apply_preset(next);
                    state.refresh_steps();
                }
                ConfigSection::Developer => {
                    state.config.neovim_preset = state.config.neovim_preset.cycle();
                    state.refresh_steps();
                }
                ConfigSection::Aesthetics => {
                    state.config.bar_layout = state.config.bar_layout.cycle();
                    state.refresh_steps();
                }
                ConfigSection::Theme => {
                    state.config.theme_name =
                        AVAILABLE_THEMES[state.theme_cursor].name.to_string();
                    state.refresh_steps();
                }
            },
            KeyCode::Enter => {
                state.refresh_steps();
                state.refresh_sudo_status();
                state.sudo_error = None;
                state.current_screen = AppScreen::Summary;
            }
            KeyCode::Esc => {
                state.current_screen = AppScreen::Welcome;
            }
            _ => {}
        },

        AppScreen::PackageSelection => {
            if state.package_search_active {
                match code {
                    KeyCode::Esc => {
                        state.package_search_active = false;
                    }
                    KeyCode::Enter => {
                        state.package_search_active = false;
                    }
                    KeyCode::Backspace => {
                        state.package_search_query.pop();
                        state.package_item_cursor = 0;
                    }
                    KeyCode::Up => {
                        state.package_item_cursor = state.package_item_cursor.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        let filtered = state.current_filtered_packages();
                        if !filtered.is_empty() && state.package_item_cursor + 1 < filtered.len() {
                            state.package_item_cursor += 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        let filtered = state.current_filtered_packages();
                        if let Some(&pkg) = filtered.get(state.package_item_cursor) {
                            state.config.toggle_package(pkg);
                        }
                    }
                    KeyCode::Char(c) if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' => {
                        state.package_search_query.push(c);
                        state.package_item_cursor = 0;
                    }
                    _ => {}
                }
                return Ok(());
            }

            let groups = state.selected_distro.package_groups();
            let total_cats = groups.len();

            match code {
                KeyCode::Char('/') => {
                    state.package_search_active = true;
                    state.package_side_packages = true;
                }
                KeyCode::Tab | KeyCode::Left | KeyCode::Right => {
                    state.package_side_packages = !state.package_side_packages;
                }
                KeyCode::Up => {
                    if !state.package_side_packages {
                        state.package_category_cursor =
                            state.package_category_cursor.saturating_sub(1);
                        state.package_item_cursor = 0;
                    } else {
                        state.package_item_cursor =
                            state.package_item_cursor.saturating_sub(1);
                    }
                }
                KeyCode::Down => {
                    if !state.package_side_packages {
                        if state.package_category_cursor + 1 < total_cats {
                            state.package_category_cursor += 1;
                            state.package_item_cursor = 0;
                        }
                    } else {
                        let filtered = state.current_filtered_packages();
                        if !filtered.is_empty() && state.package_item_cursor + 1 < filtered.len() {
                            state.package_item_cursor += 1;
                        }
                    }
                }
                KeyCode::Char(' ') => {
                    if !state.package_side_packages {
                        state.package_side_packages = true;
                    } else {
                        let filtered = state.current_filtered_packages();
                        if let Some(&pkg) = filtered.get(state.package_item_cursor) {
                            state.config.toggle_package(pkg);
                        }
                    }
                }
                // Global Select All packages
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    state.config.enable_all_global();
                }
                // Global Deselect All packages (enables selecting certain packages from zero)
                KeyCode::Char('x') | KeyCode::Char('X') => {
                    let all = state.selected_distro.all_packages();
                    state.config.disable_all_global(all);
                }
                // Category-level or Filtered Select All
                KeyCode::Char('a') => {
                    let filtered = state.current_filtered_packages();
                    state
                        .config
                        .enable_all_packages(filtered.iter().copied());
                }
                KeyCode::Char('A') => {
                    // Shift+A: Select ALL packages globally
                    state.config.enable_all_global();
                }
                // Category-level or Filtered Deselect All
                KeyCode::Char('n') => {
                    let filtered = state.current_filtered_packages();
                    state
                        .config
                        .disable_all_packages(filtered.iter().copied());
                }
                KeyCode::Char('N') => {
                    // Shift+N: Deselect ALL packages globally
                    let all = state.selected_distro.all_packages();
                    state.config.disable_all_global(all);
                }
                // Category or Filtered Invert
                KeyCode::Char('i') => {
                    let filtered = state.current_filtered_packages();
                    for &pkg in &filtered {
                        state.config.toggle_package(pkg);
                    }
                }
                KeyCode::Char('I') => {
                    // Shift+I: Invert ALL packages globally
                    let all = state.selected_distro.all_packages();
                    state.config.invert_all_global(all);
                }
                KeyCode::Esc => {
                    if !state.package_search_query.is_empty() {
                        state.package_search_query.clear();
                        state.package_item_cursor = 0;
                    } else {
                        state.refresh_steps();
                        state.current_screen = AppScreen::Configuration;
                    }
                }
                KeyCode::Enter | KeyCode::Char('b') | KeyCode::Char('B') => {
                    state.refresh_steps();
                    state.current_screen = AppScreen::Configuration;
                }
                _ => {}
            }
        }

        AppScreen::Diagnostics => match code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('I') => {
                state.current_screen = AppScreen::Configuration;
            }
            _ => {}
        },

        AppScreen::Summary => match code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                trigger_installation(terminal, state)?;
            }
            KeyCode::Esc | KeyCode::Char('b') | KeyCode::Char('B') => {
                state.sudo_error = None;
                state.current_screen = AppScreen::Configuration;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                state.should_exit = true;
            }
            _ => {}
        },

        AppScreen::Execution => match code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                state.auto_scroll_logs = !state.auto_scroll_logs;
            }
            KeyCode::Up | KeyCode::PageUp => {
                state.auto_scroll_logs = false;
                state.log_scroll = state.log_scroll.saturating_sub(2);
            }
            KeyCode::Down | KeyCode::PageDown => {
                state.log_scroll =
                    (state.log_scroll + 2).min(state.logs.len().saturating_sub(1));
            }
            _ => {}
        },

        AppScreen::Complete => match code {
            KeyCode::Char('v') | KeyCode::Char('V') => {
                state.current_screen = AppScreen::Diagnostics;
            }
            KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('Q') => {
                state.should_exit = true;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                println!("\nRebooting system...");
                let _ = std::process::Command::new("reboot").spawn();
                state.should_exit = true;
            }
            _ => {}
        },
    }

    Ok(())
}
