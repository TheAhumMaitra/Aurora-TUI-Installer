// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;
mod distro;
mod engine;
mod model;
mod ui;

use clap::Parser;
use cli::{CliArgs, DistroArg, ModeArg, PresetArg};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use distro::{DistroFamily, DistroInfo};
use model::config::{Component, InstallConfig, InstallMode, InstallPreset};
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
    if let Some(theme) = args.theme {
        config.theme_name = theme;
    }
    if let Some(dir) = args.dir {
        config.source_dir = dir;
    }

    let mut state = AppState::new(distro_info, config);

    // If explicit distro requested via CLI
    if let Some(distro_arg) = args.distro {
        let forced = match distro_arg {
            DistroArg::Arch => DistroFamily::Arch,
            DistroArg::Fedora => DistroFamily::Fedora,
            DistroArg::NixOS => DistroFamily::NixOS,
        };
        state.set_distro(forced);
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

fn run_headless(state: &mut AppState) {
    println!("=== Aurora Non-Interactive Installation ===");
    println!("Target Distribution: {}", state.selected_distro.name());
    println!("Installation Mode:   {:?}", state.config.mode);
    println!("Theme:               {}", state.config.theme_name);
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
    if modifiers.contains(KeyModifiers::CONTROL) && (code == KeyCode::Char('c') || code == KeyCode::Char('C')) {
        state.should_exit = true;
        return;
    }

    match state.current_screen {
        AppScreen::Welcome => match code {
            KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right | KeyCode::Left => {
                state.cycle_distro();
            }
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
                    ConfigSection::Preset => ConfigSection::Mode,
                    ConfigSection::Mode => ConfigSection::Theme,
                    ConfigSection::Theme => ConfigSection::DryRun,
                    ConfigSection::DryRun => ConfigSection::Components,
                    ConfigSection::Components => ConfigSection::Preset,
                };
            }
            KeyCode::BackTab => {
                state.active_config_section = match state.active_config_section {
                    ConfigSection::Preset => ConfigSection::Components,
                    ConfigSection::Mode => ConfigSection::Preset,
                    ConfigSection::Theme => ConfigSection::Mode,
                    ConfigSection::DryRun => ConfigSection::Theme,
                    ConfigSection::Components => ConfigSection::DryRun,
                };
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
            KeyCode::Char('m') | KeyCode::Char('M') => {
                state.config.mode = match state.config.mode {
                    InstallMode::Stable => InstallMode::Git,
                    InstallMode::Git => InstallMode::Stable,
                };
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
                    state.config.theme_name = AVAILABLE_THEMES[state.theme_cursor].name.to_string();
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
                        state.config.theme_name = AVAILABLE_THEMES[state.theme_cursor].name.to_string();
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
                ConfigSection::DryRun => {
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
                ConfigSection::Mode => {
                    state.config.mode = match state.config.mode {
                        InstallMode::Stable => InstallMode::Git,
                        InstallMode::Git => InstallMode::Stable,
                    };
                    state.refresh_steps();
                }
                ConfigSection::Theme => {
                    state.config.theme_name = AVAILABLE_THEMES[state.theme_cursor].name.to_string();
                    state.refresh_steps();
                }
            },
            KeyCode::Enter => {
                state.refresh_steps();
                state.current_screen = AppScreen::Summary;
            }
            KeyCode::Esc | KeyCode::Char('b') | KeyCode::Char('B') => {
                state.current_screen = AppScreen::Welcome;
            }
            _ => {}
        },

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
                state.log_scroll = (state.log_scroll + 2).min(state.logs.len().saturating_sub(1));
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
