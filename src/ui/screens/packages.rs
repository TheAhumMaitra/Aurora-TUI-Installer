// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::arch::ARCH_PACKAGE_GROUPS;
use crate::distro::DistroPackageGroup;
use crate::model::state::AppState;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let groups = ARCH_PACKAGE_GROUPS;

    render_categories(f, state, groups, chunks[0]);
    render_packages(f, state, groups, chunks[1]);
}

fn render_categories(
    f: &mut Frame,
    state: &AppState,
    groups: &[DistroPackageGroup],
    area: Rect,
) {
    let is_focused = !state.package_side_packages;
    let block = if is_focused {
        active_block(" Package Categories [Tab to switch, ↑/↓ to browse] ")
    } else {
        default_block(" Package Categories ")
    }
    .padding(Padding::uniform(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    for (i, group) in groups.iter().enumerate() {
        let is_selected = i == state.package_category_cursor;

        // Calculate enabled packages in this group
        let total = group.packages.len();
        let enabled = group
            .packages
            .iter()
            .filter(|&&p| state.config.is_package_enabled(p))
            .count();

        let pointer = if is_selected && is_focused {
            Span::styled("▶ ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD))
        } else if is_selected {
            Span::styled("• ", Style::default().fg(COLOR_MAGENTA))
        } else {
            Span::raw("  ")
        };

        let style = if is_selected {
            Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT)
        };

        let count_color = if enabled == total {
            COLOR_GREEN
        } else if enabled == 0 {
            COLOR_DARK
        } else {
            COLOR_YELLOW
        };

        lines.push(Line::from(vec![
            pointer,
            Span::styled(group.category, style),
            Span::styled(format!(" [{}/{}]", enabled, total), Style::default().fg(count_color)),
        ]));
        lines.push(Line::from(Span::styled(
            format!("    {}", group.description),
            Style::default().fg(COLOR_DARK),
        )));
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}

fn render_packages(
    f: &mut Frame,
    state: &AppState,
    groups: &[DistroPackageGroup],
    area: Rect,
) {
    let is_focused = state.package_side_packages;
    let active_group = groups.get(state.package_category_cursor);

    let title = if let Some(group) = active_group {
        format!(" Packages in {} [Space to toggle] ", group.category)
    } else {
        " Packages [Space to toggle] ".to_string()
    };

    let block = if is_focused {
        active_block(title)
    } else {
        default_block(title)
    }
    .padding(Padding::uniform(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),    // Package list
            Constraint::Length(4), // Bulk action bar
        ])
        .split(inner);

    let mut lines = Vec::new();

    if let Some(group) = active_group {
        for (i, &pkg) in group.packages.iter().enumerate() {
            let is_selected = i == state.package_item_cursor;
            let is_enabled = state.config.is_package_enabled(pkg);

            let pointer = if is_selected && is_focused {
                Span::styled("▶ ", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("  ")
            };

            let check = if is_enabled {
                Span::styled("[✓] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[ ] ", Style::default().fg(COLOR_DARK))
            };

            let style = if is_selected && is_focused {
                Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)
            } else if is_enabled {
                Style::default().fg(COLOR_TEXT)
            } else {
                Style::default().fg(COLOR_DARK)
            };

            lines.push(Line::from(vec![
                pointer,
                check,
                Span::styled(pkg, style),
            ]));
        }
    }

    // Scroll if packages exceed viewport
    let height = chunks[0].height as usize;
    let scroll_y = if state.package_item_cursor >= height {
        (state.package_item_cursor - height + 1) as u16
    } else {
        0
    };

    let p = Paragraph::new(lines).scroll((scroll_y, 0));
    f.render_widget(p, chunks[0]);

    // Bulk action bar
    let all_pkgs = crate::distro::arch::all_arch_packages();
    let total_all = all_pkgs.len();
    let enabled_all = all_pkgs.iter().filter(|&&p| state.config.is_package_enabled(p)).count();
    let excluded_all = total_all.saturating_sub(enabled_all);

    let stats_span = if excluded_all == 0 {
        Span::styled(format!("All {} Packages Active (Full Rice)", total_all), Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(format!("{}/{} Packages Active ({} Excluded)", enabled_all, total_all, excluded_all), Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD))
    };

    let action_lines = vec![
        Line::from(vec![
            Span::styled("Category: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("[Space] ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled("Toggle  ", Style::default().fg(COLOR_TEXT)),
            Span::styled("[A] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Select Cat  ", Style::default().fg(COLOR_TEXT)),
            Span::styled("[N] ", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
            Span::styled("Deselect Cat  ", Style::default().fg(COLOR_TEXT)),
            Span::styled("[I] ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Invert Cat", Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("GLOBAL:   ", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD)),
            Span::styled("[U / Shift+A] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("SELECT ALL PACKAGES   ", Style::default().fg(COLOR_WHITE)),
            Span::styled("[X / Shift+N] ", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
            Span::styled("DESELECT ALL PACKAGES", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(COLOR_CYAN)),
            stats_span,
            Span::raw("   •   "),
            Span::styled("[Enter / Esc] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Save Selection & Return", Style::default().fg(COLOR_WHITE)),
        ]),
    ];

    let action_p = Paragraph::new(action_lines)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(COLOR_DARK)),
        );
    f.render_widget(action_p, chunks[1]);
}
