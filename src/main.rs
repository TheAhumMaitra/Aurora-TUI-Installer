// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod distro;
mod engine;
mod model;
mod ui;

use clap::Parser;
use cli::{
    AudioArg, BarLayoutArg, BrowserArg, CliArgs, DisplayManagerArg, FontArg, LauncherArg,
    LockscreenArg, ModeArg, NeovimArg, PresetArg, ShellArg, TerminalArg, TweaksArg,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use distro::DistroInfo;
use model::config::{
    AudioSuiteChoice, BarLayoutChoice, BrowserChoice, Component, DisplayManagerChoice,
    FontChoice, InstallConfig, InstallMode, InstallPreset, KernelTweakProfile, LauncherStyle,
    LockscreenChoice, NeovimPreset, ShellChoice, TerminalChoice,
};
use model::state::{AppScreen, AppState, ConfigSection};
use model::theme::AVAILABLE_THEMES;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // System detection
    let distro_info = DistroInfo::detect();

    // Prepare default config
    let mut config = InstallConfig::default();

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
    if let Some(explicit_pkgs) = args.packages {
        let all = distro::arch::all_arch_packages();
        config.disable_all_global(all);
        config.preset = InstallPreset::Custom;
        for pkg in explicit_pkgs {
            config.enable_package(&pkg);
        }
    }
    if let Some(excluded) = args.exclude_packages {
        for pkg in excluded {
            config.disable_package(&pkg);
        }
    }
    if args.gaming && !config.selected_components.contains(&Component::GamingSuite) {
        config.selected_components.push(Component::GamingSuite);
    }
    if let Some(theme) = args.theme {
        config.theme_name = theme;
    }
    if let Some(dir) = args.dir {
        config.source_dir = dir;
    }

    let mut state = AppState::new(distro_info, config);

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

fn run_headless(state: &mut AppState) {
    println!("=== Aurora Non-Interactive Installation ===");
    println!("Detected System:     {}", state.distro_info.arch_flavor);
    println!(
        "Arch Compatibility:  {}",
        if state.distro_info.is_arch_compatible {
            "Compatible (Arch-based)"
        } else {
            "Incompatible (Non-Arch)"
        }
    );
    println!(
        "Package Manager:     {}",
        if state.distro_info.pacman_available {
            "pacman (found)"
        } else {
            "none"
        }
    );
    println!(
        "AUR Helper:          {}",
        state
            .distro_info
            .aur_helper
            .as_deref()
            .unwrap_or("none (will bootstrap yay)")
    );
    println!("Installation Mode:   {:?}", state.config.mode);
    println!("Neovim Preset:       {}", state.config.neovim_preset.label());
    println!("Terminal Choice:     {}", state.config.terminal_choice.label());
    println!("Web Browser:         {}", state.config.browser_choice.label());
    println!("Display Manager:     {}", state.config.display_manager.label());
    println!("Lockscreen Daemon:   {}", state.config.lockscreen.label());
    println!("Monospace Font:      {}", state.config.font_choice.label());
    println!("Waybar Layout:       {}", state.config.bar_layout.label());
    println!("Rofi Launcher:       {}", state.config.launcher_style.label());
    println!("Audio Suite:         {}", state.config.audio_suite.label());
    println!("Kernel Tweaks:       {}", state.config.kernel_tweaks.label());
    println!("Shell Choice:        {}", state.config.shell_choice.label());
    println!("Theme:               {}", state.config.theme_name);
    let all_pkgs = distro::arch::all_arch_packages();
    let enabled_count = all_pkgs.iter().filter(|&&p| state.config.is_package_enabled(p)).count();
    println!("Active Packages:     {}/{} selected", enabled_count, all_pkgs.len());
    println!("Dry Run:             {}", state.config.dry_run);
    println!("===========================================");

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
                    handle_key_event(state, key.code, key.modifiers);
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

fn handle_key_event(state: &mut AppState, code: KeyCode, modifiers: KeyModifiers) {
    // Global exit shortcut
    if modifiers.contains(KeyModifiers::CONTROL)
        && (code == KeyCode::Char('c') || code == KeyCode::Char('C'))
    {
        state.should_exit = true;
        return;
    }

    match state.current_screen {
        AppScreen::Welcome => match code {
            KeyCode::Enter => {
                state.current_screen = AppScreen::Configuration;
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
                let all = distro::arch::all_arch_packages();
                state.config.disable_all_global(all);
                state.refresh_steps();
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let next = match state.config.preset {
                    InstallPreset::FullRice => InstallPreset::MinimalRice,
                    InstallPreset::MinimalRice => InstallPreset::Custom,
                    InstallPreset::Custom => InstallPreset::FullRice,
                };
                state.config.apply_preset(next);
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
                    let next = match state.config.preset {
                        InstallPreset::FullRice => InstallPreset::MinimalRice,
                        InstallPreset::MinimalRice => InstallPreset::Custom,
                        InstallPreset::Custom => InstallPreset::FullRice,
                    };
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
                return;
            }

            let groups = distro::arch::ARCH_PACKAGE_GROUPS;
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
                    let all = distro::arch::all_arch_packages();
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
                    let all = distro::arch::all_arch_packages();
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
                    let all = distro::arch::all_arch_packages();
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

        AppScreen::Summary => match code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                state.start_installation();
            }
            KeyCode::Esc | KeyCode::Char('b') | KeyCode::Char('B') => {
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
}
