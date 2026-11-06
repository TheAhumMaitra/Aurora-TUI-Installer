// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::model::state::AppState;
use crate::ui::theme::*;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left pane: Summary Table & Confirmation Box
    render_summary_card(f, state, chunks[0]);

    // Right pane: Pipeline preview
    render_pipeline_preview(f, state, chunks[1]);
}

fn render_summary_card(f: &mut Frame, state: &AppState, area: Rect) {
    let block = active_block(" Review Installation Settings ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(12),   // Settings table
            Constraint::Length(7), // Confirmation Box
        ])
        .split(inner);

    let mode_str = state.config.mode.label();
    let dry_status = if state.config.dry_run {
        Span::styled("ENABLED (Safe Preview Mode - No real changes)", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("DISABLED (Live System Installation)", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD))
    };

    let backup_path = state.config.backup_dir.display().to_string();

    let disabled_cnt = state.config.disabled_packages.len();
    let pkg_custom_str = if disabled_cnt > 0 {
        format!("{} components active ({} packages excluded)", state.config.selected_components.len(), disabled_cnt)
    } else {
        format!("{} components active (All packages enabled)", state.config.selected_components.len())
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Target System:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(&state.distro_info.distro_flavor, Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" [{} • {}]", state.selected_distro.label(), state.selected_distro.package_manager()), Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Preset Profile:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.config.preset.label(), Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled(format!(" ({})", mode_str), Style::default().fg(COLOR_DARK)),
            Span::raw("  •  "),
            Span::styled(pkg_custom_str, Style::default().fg(COLOR_GREEN)),
        ]),
        Line::from(vec![
            Span::styled("Developer Suite:   ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.config.neovim_preset.label(), Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::raw("  •  "),
            Span::styled(state.config.terminal_choice.label(), Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Web & Display:     ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.config.browser_choice.short_name(), Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::raw("  •  Login: "),
            Span::styled(state.config.display_manager.short_name(), Style::default().fg(COLOR_YELLOW)),
        ]),
        Line::from(vec![
            Span::styled("Rice Aesthetics:   ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(format!("Bar: {}", state.config.bar_layout.short_name()), Style::default().fg(COLOR_WHITE)),
            Span::raw("  •  "),
            Span::styled(format!("Launcher: {}", state.config.launcher_style.short_name()), Style::default().fg(COLOR_MAGENTA)),
            Span::raw("  •  "),
            Span::styled(format!("Theme: {}", state.config.theme_name), Style::default().fg(COLOR_CYAN)),
        ]),
        Line::from(vec![
            Span::styled("Sound & Kernel:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.config.audio_suite.label(), Style::default().fg(COLOR_WHITE)),
            Span::raw("  •  "),
            Span::styled(state.config.kernel_tweaks.short_name(), Style::default().fg(COLOR_GREEN)),
        ]),
        Line::from(vec![
            Span::styled("Shell & Gaming:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.config.shell_choice.label(), Style::default().fg(COLOR_WHITE)),
            Span::raw("  •  Gaming Suite: "),
            Span::styled(
                if state.config.is_component_selected(crate::model::config::Component::GamingSuite) { "Active" } else { "Disabled" },
                Style::default().fg(if state.config.is_component_selected(crate::model::config::Component::GamingSuite) { COLOR_GREEN } else { COLOR_DARK }),
            ),
        ]),
        Line::from(vec![
            Span::styled("Dry-Run Status:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            dry_status,
        ]),
        Line::from(vec![
            Span::styled("Backup Location:   ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(backup_path, Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Memory-Safe Sudo:  ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(
                if state.config.allow_sudo_rs { "Enabled (sudo-rs)" } else { "Disabled" },
                Style::default().fg(COLOR_TEXT),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled("Note: Installation requires sudo for system package operations.", Style::default().fg(COLOR_DARK))),
    ];

    let p = Paragraph::new(lines);
    f.render_widget(p, chunks[0]);

    // Confirmation box
    let confirm_lines = vec![
        Line::from(Span::styled("READY TO PROCEED?", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ Enter ]", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" or ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ Y ]", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" to launch the installation pipeline.", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ Esc ]", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled(" or ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ B ]", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled(" to return to Configuration and change settings.", Style::default().fg(COLOR_WHITE)),
        ]),
    ];

    let confirm_p = Paragraph::new(confirm_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_MAGENTA))
                .padding(Padding::vertical(1)),
        );
    f.render_widget(confirm_p, chunks[1]);
}

fn render_pipeline_preview(f: &mut Frame, state: &AppState, area: Rect) {
    let block = default_block(" Execution Pipeline Plan ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        format!("Total Planned Steps: {}", state.steps.len()),
        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (i, step) in state.steps.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(format!("{:2}. ", i + 1), Style::default().fg(COLOR_DARK)),
            Span::styled(&step.title, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(&step.description, Style::default().fg(COLOR_DARK)),
        ]));
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}
