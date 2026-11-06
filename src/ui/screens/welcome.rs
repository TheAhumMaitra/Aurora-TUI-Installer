// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::model::state::AppState;
use crate::ui::theme::*;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

const LOGO_TEXT: &str = r#"
   ▄████████  ███    █▄     ▄████████  ▄██████▄     ▄████████    ▄████████ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███  ███    ███  ▄███▄▄▄▄██▀ ███    ███  ▄███▄▄▄▄██▀   ███    ███ 
▀███████████  ███    ███ ▀▀███▀▀▀▀▀   ███    ███ ▀▀███▀▀▀▀▀   ▀███████████ 
  ███    ███  ███    ███ ▀███████████ ███    ███ ▀███████████   ███    ███ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    █▀   ████████▀    ███    ███  ▀██████▀    ███    ███   ███    █▀  
"#;

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left pane: Logo & ArchOS Compatibility Verification
    render_left_pane(f, state, chunks[0]);

    // Right pane: System Diagnostics Card
    render_right_pane(f, state, chunks[1]);
}

fn render_left_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let block = active_block(" Welcome to Aurora ")
        .padding(Padding::uniform(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // Logo
            Constraint::Length(3),  // Subtitle
            Constraint::Min(8),     // ArchOS Compatibility Card
            Constraint::Length(3),  // Action notice
        ])
        .split(inner);

    // Logo
    let mut logo_lines = Vec::new();
    for (i, line) in LOGO_TEXT.lines().enumerate() {
        let color = if i < 3 {
            COLOR_MAGENTA
        } else if i < 6 {
            COLOR_BLUE
        } else {
            COLOR_CYAN
        };
        logo_lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )));
    }
    let logo_p = Paragraph::new(logo_lines).alignment(Alignment::Center);
    f.render_widget(logo_p, chunks[0]);

    // Subtitle
    let desc_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("AURORA ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("• A minimal, elegant Hyprland rice", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(Span::styled(
            "Tailored for NixOS, Fedora and Fedora-based distros, Arch and Arch-based distros.",
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(desc_p, chunks[1]);

    // Distribution Compatibility Verification Box
    let info = &state.distro_info;
    let (border_color, title_str) = match state.selected_distro {
        crate::distro::DistroFamily::Arch => (
            COLOR_GREEN,
            format!(" {} Arch Linux / Arch-based System Verified ", state.selected_distro.icon()),
        ),
        crate::distro::DistroFamily::Fedora => (
            COLOR_BLUE,
            format!(" {} Fedora Linux / Fedora-based System Verified ", state.selected_distro.icon()),
        ),
        crate::distro::DistroFamily::NixOS => (
            COLOR_CYAN,
            format!(" {} NixOS System Verified ", state.selected_distro.icon()),
        ),
        crate::distro::DistroFamily::Unknown => (
            COLOR_RED,
            format!(" {} Distribution Compatibility Warning ", state.selected_distro.icon()),
        ),
    };
    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(title_str)
        .title_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD))
        .padding(Padding::uniform(1));

    let card_lines = match state.selected_distro {
        crate::distro::DistroFamily::Arch => vec![
            Line::from(vec![
                Span::styled(" ✔ ARCH-BASED SYSTEM READY ", Style::default().fg(COLOR_WHITE).bg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("Arch / Pacman / AUR Pipeline", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Distribution:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(&info.distro_flavor, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" [Family: {}]", state.selected_distro.name()), Style::default().fg(COLOR_DARK)),
            ]),
            Line::from(vec![
                Span::styled("Package Manager:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(
                    if info.pacman_available { "pacman (ready)" } else { "pacman (required)" },
                    Style::default().fg(COLOR_GREEN),
                ),
                Span::raw("  •  AUR Helper: "),
                Span::styled(
                    match &info.aur_helper {
                        Some(h) => format!("{} (ready)", h),
                        None => "yay (auto-bootstrapped)".to_string(),
                    },
                    Style::default().fg(COLOR_YELLOW),
                ),
            ]),
            Line::from(vec![
                Span::styled("Rice Support:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("Hyprland, Waybar, SwayNC, Quickshell & SDDM mapped", Style::default().fg(COLOR_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Safety Assurance: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("~/.config/hypr/User custom dotfiles are preserved", Style::default().fg(COLOR_GREEN)),
            ]),
        ],
        crate::distro::DistroFamily::Fedora => vec![
            Line::from(vec![
                Span::styled(" ✔ FEDORA-BASED SYSTEM READY ", Style::default().fg(COLOR_WHITE).bg(COLOR_BLUE).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("Fedora / DNF / COPR Pipeline", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Distribution:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(&info.distro_flavor, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" [Family: {}]", state.selected_distro.name()), Style::default().fg(COLOR_DARK)),
            ]),
            Line::from(vec![
                Span::styled("Package Manager:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(
                    if info.dnf_available { "dnf (ready)" } else { "dnf (required)" },
                    Style::default().fg(COLOR_GREEN),
                ),
                Span::raw("  •  COPRs: "),
                Span::styled("Hyprland, Zen, Fonts, Starship, SwayOSD", Style::default().fg(COLOR_YELLOW)),
            ]),
            Line::from(vec![
                Span::styled("Desktop Addons:   ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("Hyprwave, Nordzy cursors, rofi-emoji & snappy-switcher", Style::default().fg(COLOR_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Display Manager:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("Disables GDM/competing DMs, enables SDDM Astronaut theme", Style::default().fg(COLOR_GREEN)),
            ]),
        ],
        crate::distro::DistroFamily::NixOS => vec![
            Line::from(vec![
                Span::styled(" ✔ NIXOS SYSTEM READY ", Style::default().fg(COLOR_WHITE).bg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("NixOS / Flakes Rebuild Pipeline", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Distribution:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(&info.distro_flavor, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" [Family: {}]", state.selected_distro.name()), Style::default().fg(COLOR_DARK)),
            ]),
            Line::from(vec![
                Span::styled("Package Manager:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled(
                    if info.nix_available { "nix flakes (ready)" } else { "nix (required)" },
                    Style::default().fg(COLOR_GREEN),
                ),
                Span::raw("  •  System Rebuild: "),
                Span::styled("nixos-rebuild switch --flake", Style::default().fg(COLOR_YELLOW)),
            ]),
            Line::from(vec![
                Span::styled("Flake Config:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("configuration.nix, home.nix, and flake.nix deployed to /etc/nixos", Style::default().fg(COLOR_TEXT)),
            ]),
            Line::from(vec![
                Span::styled("Dev Shell:        ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("Compiles Aurora scripts & waytrogen in nix dev shell", Style::default().fg(COLOR_GREEN)),
            ]),
        ],
        crate::distro::DistroFamily::Unknown => vec![
            Line::from(vec![
                Span::styled(" ✖ UNSUPPORTED DISTRIBUTION ", Style::default().fg(COLOR_WHITE).bg(COLOR_RED).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("Incompatible", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Detected OS:     ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
                Span::styled(&info.pretty_name, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" [ID: {}]", info.id), Style::default().fg(COLOR_DARK)),
            ]),
            Line::from(vec![
                Span::styled("Notice:          ", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
                Span::styled(
                    "Aurora officially supports NixOS, Fedora and Fedora-based distros, Arch and Arch-based distros.",
                    Style::default().fg(COLOR_TEXT),
                ),
            ]),
            Line::from(Span::styled(
                "You can press [D] to switch target distro mode, or run with --dry-run.",
                Style::default().fg(COLOR_DARK),
            )),
        ],
    };

    let card_p = Paragraph::new(card_lines).block(card_block);
    f.render_widget(card_p, chunks[2]);

    // Notice & Next Action
    let notice = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("➜ Press ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ Enter ]", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" to configure options  •  Press ", Style::default().fg(COLOR_WHITE)),
            Span::styled("[ D ]", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" to switch target distro (Current: {})", state.selected_distro.name()), Style::default().fg(COLOR_YELLOW)),
        ]),
        Line::from(vec![
            Span::styled("💡 Tip: ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled(
                "Dry-run simulation is available anytime to test the entire installation safely without modifying files.",
                Style::default().fg(COLOR_DARK),
            ),
        ]),
    ]);
    f.render_widget(notice, chunks[3]);
}

fn render_right_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let block = default_block(" System Diagnostics ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let info = &state.distro_info;

    let root_warning = if info.is_root {
        Span::styled(" ✗ Running as root (Not recommended!)", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" ✓ Running as standard user", Style::default().fg(COLOR_GREEN))
    };

    let disk_status = if info.home_free_gb >= 5.0 {
        Span::styled(
            format!(" ✓ {:.1} GB free (Adequate)", info.home_free_gb),
            Style::default().fg(COLOR_GREEN),
        )
    } else {
        Span::styled(
            format!(" ▲ {:.1} GB free (Recommended >= 5 GB)", info.home_free_gb),
            Style::default().fg(COLOR_YELLOW),
        )
    };

    let git_status = if info.git_available {
        Span::styled(" ✓ Installed", Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled(" ▲ Not found (Will be bootstrapped)", Style::default().fg(COLOR_YELLOW))
    };

    let cargo_status = if info.cargo_available {
        Span::styled(" ✓ Installed", Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled(" ▲ Not found", Style::default().fg(COLOR_YELLOW))
    };

    let hypr_status = if info.hyprland_running {
        Span::styled(" Active Wayland / Hyprland Session", Style::default().fg(COLOR_CYAN))
    } else {
        Span::styled(" Not running (Headless / TTY / Other)", Style::default().fg(COLOR_DARK))
    };

    let pm_status = match state.selected_distro {
        crate::distro::DistroFamily::Arch => {
            let aur = match &info.aur_helper {
                Some(h) => format!("✓ Pacman & {}", h),
                None => "✓ Pacman (yay will bootstrap)".to_string(),
            };
            Span::styled(aur, Style::default().fg(COLOR_GREEN))
        }
        crate::distro::DistroFamily::Fedora => {
            if info.dnf_available {
                Span::styled("✓ DNF & RPM active", Style::default().fg(COLOR_GREEN))
            } else {
                Span::styled("▲ DNF not found", Style::default().fg(COLOR_YELLOW))
            }
        }
        crate::distro::DistroFamily::NixOS => {
            if info.nix_available {
                Span::styled("✓ Nix & Nix Flakes active", Style::default().fg(COLOR_GREEN))
            } else {
                Span::styled("▲ Nix not found in PATH", Style::default().fg(COLOR_YELLOW))
            }
        }
        crate::distro::DistroFamily::Unknown => {
            Span::styled("▲ Unsupported package manager", Style::default().fg(COLOR_RED))
        }
    };

    let diag_lines = vec![
        Line::from(vec![
            Span::styled("Detected OS:        ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(&info.distro_flavor, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" (ID: {})", info.id), Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Target Distro:      ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(
                state.selected_distro.name(),
                Style::default().fg(if state.selected_distro != crate::distro::DistroFamily::Unknown { COLOR_GREEN } else { COLOR_RED }).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Package Manager:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            pm_status,
        ]),
        Line::from(vec![
            Span::styled("Kernel & Arch:      ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} ({})", info.kernel, info.arch), Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("CPU & Cores:        ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} logical cores", info.cpu_cores), Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("System Memory:      ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{:.1} GB Available / {:.1} GB Total", info.available_ram_gb, info.total_ram_gb),
                Style::default().fg(COLOR_TEXT),
            ),
        ]),
        Line::from(vec![
            Span::styled("Free Disk ($HOME):  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            disk_status,
        ]),
        Line::from(vec![
            Span::styled("User & Privileges:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(&info.current_user, Style::default().fg(COLOR_WHITE)),
            root_warning,
        ]),
        Line::from(""),
        Line::from(Span::styled("─ Toolchain Diagnostics ────────────────────────", Style::default().fg(COLOR_DARK))),
        Line::from(vec![
            Span::styled("Git:                ", Style::default().fg(COLOR_CYAN)),
            git_status,
        ]),
        Line::from(vec![
            Span::styled("Cargo / Rust:       ", Style::default().fg(COLOR_CYAN)),
            cargo_status,
        ]),
        Line::from(vec![
            Span::styled("Hyprland Session:   ", Style::default().fg(COLOR_CYAN)),
            hypr_status,
        ]),
    ];

    let diag_p = Paragraph::new(diag_lines);
    f.render_widget(diag_p, inner);
}
