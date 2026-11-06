// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};

pub const COLOR_CYAN: Color = Color::Rgb(125, 207, 255);
pub const COLOR_MAGENTA: Color = Color::Rgb(187, 154, 247);
pub const COLOR_BLUE: Color = Color::Rgb(122, 162, 247);
pub const COLOR_GREEN: Color = Color::Rgb(158, 206, 106);
pub const COLOR_YELLOW: Color = Color::Rgb(224, 175, 104);
pub const COLOR_RED: Color = Color::Rgb(247, 118, 142);
pub const COLOR_DARK: Color = Color::Rgb(86, 95, 137);
pub const COLOR_TEXT: Color = Color::Rgb(192, 202, 245);
pub const COLOR_WHITE: Color = Color::Rgb(255, 255, 255);

pub fn style_header() -> Style {
    Style::default()
        .fg(COLOR_CYAN)
        .add_modifier(Modifier::BOLD)
}

pub fn default_block<'a>(title: impl Into<Line<'a>>) -> Block<'a> {
    Block::default()
        .title(title.into())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_DARK))
}

pub fn active_block<'a>(title: impl Into<Line<'a>>) -> Block<'a> {
    Block::default()
        .title(title.into())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN))
}
