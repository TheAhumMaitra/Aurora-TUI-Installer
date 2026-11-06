// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::model::config::Component;
use crate::model::state::{AppState, ConfigSection};
use crate::model::theme::AVAILABLE_THEMES;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left pane: Presets, Modes, Theme Picker & Flags
    render_left_pane(f, state, chunks[0]);

    // Right pane: Components Checklist & Details
    render_right_pane(f, state, chunks[1]);
}

fn render_left_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Preset selection
            Constraint::Length(5),  // Mode selection
            Constraint::Min(8),     // Theme selector
            Constraint::Length(5),  // Dry-run & sudo-rs toggles
        ])
        .split(area);

    // 1. Preset Box
    let preset_active = state.active_config_section == ConfigSection::Preset;
    let preset_block = if preset_active {
        active_block(" 1. Preset [Tab to navigate, P to cycle] ")
    } else {
        default_block(" 1. Preset ")
    };
    let preset_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Current Preset: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.preset.label(),
                Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            state.config.preset.description(),
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .block(preset_block.padding(Padding::horizontal(1)));
    f.render_widget(preset_p, chunks[0]);

    // 2. Mode Box
    let mode_active = state.active_config_section == ConfigSection::Mode;
    let mode_block = if mode_active {
        active_block(" 2. Release Channel [M to cycle] ")
    } else {
        default_block(" 2. Release Channel ")
    };
    let mode_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Install Mode: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.mode.label(),
                Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            state.config.mode.description(),
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .block(mode_block.padding(Padding::horizontal(1)));
    f.render_widget(mode_p, chunks[1]);

    // 3. Theme Selector
    let theme_active = state.active_config_section == ConfigSection::Theme;
    let theme_block = if theme_active {
        active_block(" 3. Default Desktop Theme [↑/↓ to choose] ")
    } else {
        default_block(" 3. Default Desktop Theme ")
    };

    let mut theme_lines = Vec::new();
    for (i, theme) in AVAILABLE_THEMES.iter().enumerate() {
        let is_current = theme.name == state.config.theme_name;
        let is_cursor = i == state.theme_cursor;

        let marker = if is_current { "● " } else { "○ " };
        let pointer = if theme_active && is_cursor { "▶ " } else { "  " };

        let style = if is_current {
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)
        } else if theme_active && is_cursor {
            Style::default().fg(COLOR_WHITE)
        } else {
            Style::default().fg(COLOR_TEXT)
        };

        theme_lines.push(Line::from(vec![
            Span::styled(pointer, Style::default().fg(COLOR_MAGENTA)),
            Span::styled(marker, Style::default().fg(COLOR_CYAN)),
            Span::styled(theme.name, style),
            Span::styled(format!(" - {}", theme.description), Style::default().fg(COLOR_DARK)),
        ]));
    }

    let theme_p = Paragraph::new(theme_lines)
        .block(theme_block.padding(Padding::horizontal(1)));
    f.render_widget(theme_p, chunks[2]);

    // 4. Safe mode / Dry Run / Sudo-rs
    let dry_run_active = state.active_config_section == ConfigSection::DryRun;
    let dry_block = if dry_run_active {
        active_block(" 4. Options & Flags [Space to toggle] ")
    } else {
        default_block(" 4. Options & Flags ")
    };

    let dry_box = if state.config.dry_run {
        Span::styled("[X] DRY-RUN MODE: Preview all actions safely without system modification", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("[ ] Real Installation: Apply packages and configs directly", Style::default().fg(COLOR_TEXT))
    };

    let sudo_box = if state.config.allow_sudo_rs {
        Span::styled("[X] Switch sudo to memory-safe sudo-rs", Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled("[ ] Keep standard sudo implementation", Style::default().fg(COLOR_DARK))
    };

    let dry_p = Paragraph::new(vec![
        Line::from(dry_box),
        Line::from(sudo_box),
    ])
    .block(dry_block.padding(Padding::horizontal(1)));
    f.render_widget(dry_p, chunks[3]);
}

fn render_right_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let comp_active = state.active_config_section == ConfigSection::Components;
    let block = if comp_active {
        active_block(" Components Checklist [↑/↓ to navigate, Space to toggle] ")
    } else {
        default_block(" Components Checklist ")
    }
    .padding(Padding::uniform(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Checkboxes list
            Constraint::Length(5), // Selected item explanation
        ])
        .split(inner);

    let mut comp_lines = Vec::new();
    for (i, &comp) in Component::ALL.iter().enumerate() {
        let is_selected = state.config.is_component_selected(comp);
        let is_cursor = i == state.component_cursor;

        let check = if is_selected {
            Span::styled("[✓] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD))
        } else {
            Span::styled("[ ] ", Style::default().fg(COLOR_DARK))
        };

        let pointer = if comp_active && is_cursor {
            Span::styled("▶ ", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD))
        } else {
            Span::raw("  ")
        };

        let label_style = if comp_active && is_cursor {
            Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)
        } else if is_selected {
            Style::default().fg(COLOR_TEXT)
        } else {
            Style::default().fg(COLOR_DARK)
        };

        comp_lines.push(Line::from(vec![
            pointer,
            check,
            Span::styled(comp.label(), label_style),
        ]));
        comp_lines.push(Line::from(""));
    }

    let comp_p = Paragraph::new(comp_lines);
    f.render_widget(comp_p, chunks[0]);

    // Active item description box
    let selected_comp = Component::ALL[state.component_cursor.min(Component::ALL.len() - 1)];
    let desc_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("About: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(selected_comp.label(), Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(Span::styled(
            selected_comp.description(),
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_DARK))
            .padding(Padding::horizontal(1)),
    );
    f.render_widget(desc_p, chunks[1]);
}
