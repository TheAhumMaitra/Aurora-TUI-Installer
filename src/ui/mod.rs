// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod screens;
pub mod theme;

use crate::model::state::{AppScreen, AppState};
use crate::ui::theme::*;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_app(f: &mut Frame, state: &AppState) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header & Breadcrumbs
            Constraint::Min(10),   // Active Screen Body
            Constraint::Length(4), // Footer & Keybinds (multi-line wrapped, zero overlap)
        ])
        .split(size);

    render_header(f, state, chunks[0]);

    match state.current_screen {
        AppScreen::Welcome => screens::welcome::render(f, state, chunks[1]),
        AppScreen::Configuration => screens::configuration::render(f, state, chunks[1]),
        AppScreen::PackageSelection => screens::packages::render(f, state, chunks[1]),
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
        let is_active = *screen == state.current_screen
            || (*screen == AppScreen::Configuration && state.current_screen == AppScreen::PackageSelection);
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

        let screen_title = if is_active && state.current_screen == AppScreen::PackageSelection {
            "2. Packages Customizer"
        } else {
            screen.title()
        };

        tab_spans.push(Span::styled(format!(" {}{} ", prefix, screen_title), style));

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
    let tag = |key: &str, desc: &str| -> Vec<Span<'static>> {
        vec![
            Span::styled(
                format!("[ {} ]", key),
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" {}  ", desc), Style::default().fg(COLOR_TEXT)),
        ]
    };

    let sep = || -> Span<'static> {
        Span::styled("• ", Style::default().fg(COLOR_DARK))
    };

    let footer_lines: Vec<Line> = match state.current_screen {
        AppScreen::Welcome => vec![
            Line::from(vec![
                tag("Enter", "Proceed to Configuration")[0].clone(),
                tag("Enter", "Proceed to Configuration")[1].clone(),
                sep(),
                tag("Q / Esc", "Quit")[0].clone(),
                tag("Q / Esc", "Quit")[1].clone(),
            ]),
        ],

        AppScreen::Configuration => {
            let mut row1 = Vec::new();
            for (k, d) in [
                ("Tab", "Switch Card"),
                ("Enter", "Review Plan"),
                ("Esc", "Back"),
                ("K", "Package Customizer"),
                ("A", "Select All Pkgs"),
                ("X", "Deselect All Pkgs"),
            ] {
                row1.extend(tag(k, d));
                row1.push(sep());
            }
            row1.pop(); // remove trailing sep

            let mut row2 = Vec::new();
            for (k, d) in [
                ("V", "Neovim"),
                ("T", "Terminal"),
                ("W", "Browser"),
                ("L", "Display Mgr"),
                ("B", "Waybar"),
                ("R", "Rofi"),
                ("C", "Lockscreen"),
                ("F", "Font"),
                ("E", "Audio DSP"),
                ("Z", "Kernel"),
                ("H", "Shell"),
                ("G", "Gaming"),
                ("O", "Motion"),
                ("D", "Dry-Run"),
                ("S", "Sudo-rs"),
            ] {
                row2.extend(tag(k, d));
                row2.push(sep());
            }
            row2.pop();

            vec![Line::from(row1), Line::from(row2)]
        }

        AppScreen::PackageSelection => {
            let mut row1 = Vec::new();
            for (k, d) in [
                ("Space", "Toggle Package"),
                ("Tab / ←/→", "Switch Pane"),
                ("/", "Search / Filter"),
                ("A", "Select Category"),
                ("N", "Deselect Category"),
                ("I", "Invert Category"),
            ] {
                row1.extend(tag(k, d));
                row1.push(sep());
            }
            row1.pop();

            let mut row2 = Vec::new();
            for (k, d) in [
                ("U / Shift+A", "SELECT ALL PACKAGES GLOBALLY"),
                ("X / Shift+N", "DESELECT ALL PACKAGES (PICK CERTAIN)"),
                ("Enter / Esc", "Save Selection & Return"),
            ] {
                row2.extend(tag(k, d));
                row2.push(sep());
            }
            row2.pop();

            vec![Line::from(row1), Line::from(row2)]
        }

        AppScreen::Summary => vec![
            Line::from(vec![
                tag("Enter / Y", "Start Installation Pipeline")[0].clone(),
                tag("Enter / Y", "Start Installation Pipeline")[1].clone(),
                sep(),
                tag("Esc / B", "Return to Configuration")[0].clone(),
                tag("Esc / B", "Return to Configuration")[1].clone(),
                sep(),
                tag("Q", "Cancel & Quit")[0].clone(),
                tag("Q", "Cancel & Quit")[1].clone(),
            ]),
        ],

        AppScreen::Execution => vec![
            Line::from(vec![
                tag("S", "Toggle Live Log Auto-Scroll")[0].clone(),
                tag("S", "Toggle Live Log Auto-Scroll")[1].clone(),
                sep(),
                tag("↑/↓ / PgUp/PgDn", "Scroll Log History")[0].clone(),
                tag("↑/↓ / PgUp/PgDn", "Scroll Log History")[1].clone(),
            ]),
        ],

        AppScreen::Complete => vec![
            Line::from(vec![
                tag("Enter / Q", "Exit Installer")[0].clone(),
                tag("Enter / Q", "Exit Installer")[1].clone(),
                sep(),
                tag("R", "Reboot System")[0].clone(),
                tag("R", "Reboot System")[1].clone(),
            ]),
        ],
    };

    let footer_p = Paragraph::new(footer_lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(COLOR_DARK)),
        );
    f.render_widget(footer_p, area);
}
