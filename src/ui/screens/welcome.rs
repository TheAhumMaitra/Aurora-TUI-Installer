// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroFamily;
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

    // Left pane: Logo & Distro Selection
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
            Constraint::Length(10), // Logo
            Constraint::Length(4),  // Description
            Constraint::Length(6),  // Distro Selector
            Constraint::Min(2),     // Notice
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
            "Unified Ratatui Installer supporting Arch Linux, Fedora, and NixOS.",
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .alignment(Alignment::Center);
    f.render_widget(desc_p, chunks[1]);

    // Distro Selector Box
    let arch_selected = state.selected_distro == DistroFamily::Arch;
    let fedora_selected = state.selected_distro == DistroFamily::Fedora;
    let nixos_selected = state.selected_distro == DistroFamily::NixOS;

    let pill = |name: String, selected: bool| -> Span {
        if selected {
            Span::styled(
                format!("  ▶ {} ◀  ", name),
                Style::default()
                    .fg(COLOR_WHITE)
                    .bg(COLOR_BLUE)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!("    {}    ", name),
                Style::default().fg(COLOR_DARK),
            )
        }
    };

    let selector_lines = vec![
        Line::from(vec![
            Span::styled("TARGET DISTRIBUTION: ", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD)),
            Span::styled("(Press [D] to cycle)", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(""),
        Line::from(vec![
            pill(format!("{} Arch Linux", DistroFamily::Arch.icon()), arch_selected),
            Span::raw(" "),
            pill(format!("{} Fedora Linux", DistroFamily::Fedora.icon()), fedora_selected),
            Span::raw(" "),
            pill(format!("{} NixOS", DistroFamily::NixOS.icon()), nixos_selected),
        ]),
    ];

    let selector_p = Paragraph::new(selector_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_DARK)),
        );
    f.render_widget(selector_p, chunks[2]);

    // Notice
    let notice = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("💡 Tip: ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled(
                "You can test the entire workflow safely using Dry-Run Mode on the next screen.",
                Style::default().fg(COLOR_TEXT),
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

    let diag_lines = vec![
        Line::from(vec![
            Span::styled("Detected OS:        ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(&info.pretty_name, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" (ID: {})", info.id), Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Distro Family:      ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(info.family.name(), Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" [{}]", info.family.package_manager()), Style::default().fg(COLOR_DARK)),
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
