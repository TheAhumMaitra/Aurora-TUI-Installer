// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod screens;
pub mod theme;

use crate::model::state::{AppScreen, AppState};
use crate::ui::theme::*;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_app(f: &mut Frame, state: &AppState) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header & Breadcrumbs
            Constraint::Min(10),   // Active Screen Body
            Constraint::Length(2), // Footer & Keybinds
        ])
        .split(size);

    render_header(f, state, chunks[0]);

    match state.current_screen {
        AppScreen::Welcome => screens::welcome::render(f, state, chunks[1]),
        AppScreen::Configuration => screens::configuration::render(f, state, chunks[1]),
        AppScreen::Summary => screens::summary::render(f, state, chunks[1]),
        AppScreen::Execution => screens::execution::render(f, state, chunks[1]),
        AppScreen::Complete => screens::complete::render(f, state, chunks[1]),
    }

    render_footer(f, state, chunks[2]);
}

fn render_header(f: &mut Frame, state: &AppState, area: Rect) {
    let screens = [
        AppScreen::Welcome,
        AppScreen::Configuration,
        AppScreen::Summary,
        AppScreen::Execution,
        AppScreen::Complete,
    ];

    let mut tab_spans = Vec::new();
    tab_spans.push(Span::styled(" AURORA INSTALLER  ", style_header()));
    tab_spans.push(Span::styled("│ ", Style::default().fg(COLOR_DARK)));

    for (i, screen) in screens.iter().enumerate() {
        let is_active = *screen == state.current_screen;
        let is_done = screen.index() < state.current_screen.index();

        let (style, prefix) = if is_active {
            (
                Style::default()
                    .fg(COLOR_CYAN)
                    .bg(COLOR_DARK)
                    .add_modifier(Modifier::BOLD),
                "▶ ",
            )
        } else if is_done {
            (
                Style::default().fg(COLOR_GREEN),
                "✓ ",
            )
        } else {
            (
                Style::default().fg(COLOR_DARK),
                "  ",
            )
        };

        tab_spans.push(Span::styled(format!(" {}{} ", prefix, screen.title()), style));

        if i < screens.len() - 1 {
            tab_spans.push(Span::styled(" ─► ", Style::default().fg(COLOR_DARK)));
        }
    }

    let header_p = Paragraph::new(Line::from(tab_spans))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(COLOR_DARK)),
        );

    f.render_widget(header_p, area);
}

fn render_footer(f: &mut Frame, state: &AppState, area: Rect) {
    let keybinds: Vec<(&str, &str)> = match state.current_screen {
        AppScreen::Welcome => vec![
            ("D", "Switch Target Distro"),
            ("Enter", "Proceed to Configuration"),
            ("Q", "Quit"),
        ],
        AppScreen::Configuration => vec![
            ("Tab", "Switch Section"),
            ("↑/↓", "Navigate"),
            ("Space", "Toggle / Select"),
            ("P", "Cycle Preset"),
            ("M", "Cycle Mode"),
            ("Enter", "Review Pipeline"),
            ("Esc", "Back"),
        ],
        AppScreen::Summary => vec![
            ("Enter / Y", "Start Installation"),
            ("Esc / B", "Return to Configuration"),
            ("Q", "Cancel & Quit"),
        ],
        AppScreen::Execution => vec![
            ("S", "Toggle Log Auto-Scroll"),
            ("↑/↓ / PgUp/PgDn", "Scroll Log History"),
        ],
        AppScreen::Complete => vec![
            ("Enter", "Exit Installer"),
            ("R", "Reboot System"),
        ],
    };

    let mut spans = Vec::new();
    for (i, (key, desc)) in keybinds.iter().enumerate() {
        spans.push(Span::styled(
            format!("[ {} ]", key),
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(format!(" {}  ", desc), Style::default().fg(COLOR_TEXT)));

        if i < keybinds.len() - 1 {
            spans.push(Span::styled("• ", Style::default().fg(COLOR_DARK)));
        }
    }

    let footer_p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    f.render_widget(footer_p, area);
}
