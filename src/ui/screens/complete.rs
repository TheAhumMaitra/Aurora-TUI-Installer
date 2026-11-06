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
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Hero banner
            Constraint::Min(10),   // Split: Post-Install Details (left) & Keybindings Cheatsheet (right)
        ])
        .split(area);

    render_hero_banner(f, state, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_results_card(f, state, body_chunks[0]);
    render_cheatsheet(f, state, body_chunks[1]);
}

fn render_hero_banner(f: &mut Frame, state: &AppState, area: Rect) {
    let (msg, color) = if state.config.dry_run {
        (
            "★ AURORA DRY-RUN SIMULATION COMPLETED ★",
            COLOR_YELLOW,
        )
    } else if state.install_success {
        (
            "✔ AURORA INSTALLATION COMPLETED SUCCESSFULLY! ✔",
            COLOR_GREEN,
        )
    } else {
        (
            "▲ AURORA INSTALLATION COMPLETED WITH WARNINGS ▲",
            COLOR_YELLOW,
        )
    };

    let p = Paragraph::new(vec![
        Line::from(Span::styled(
            msg,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            &state.summary_message,
            Style::default().fg(COLOR_TEXT),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .padding(Padding::vertical(1)),
    );

    f.render_widget(p, area);
}

fn render_results_card(f: &mut Frame, state: &AppState, area: Rect) {
    let block = active_block(" Post-Installation Details ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let backup_dir = state.config.backup_dir.display().to_string();

    let lines = vec![
        Line::from(vec![
            Span::styled("Target Distro:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(state.selected_distro.name(), Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Applied Theme:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(&state.config.theme_name, Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Config Backup:    ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(backup_dir, Style::default().fg(COLOR_YELLOW)),
        ]),
        Line::from(""),
        Line::from(Span::styled("─ Next Steps ───────────────────────────────────", Style::default().fg(COLOR_DARK))),
        Line::from(""),
        Line::from(vec![
            Span::styled("1. Reload Shell: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("exec $SHELL", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(Span::styled("   Ensures newly installed binaries in ~/.cargo/bin are available.", Style::default().fg(COLOR_DARK))),
        Line::from(""),
        Line::from(vec![
            Span::styled("2. Start Hyprland: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Select 'Hyprland' at login screen", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(Span::styled("   Or log out of your current session and let SDDM launch Aurora.", Style::default().fg(COLOR_DARK))),
        Line::from(""),
        Line::from(vec![
            Span::styled("3. Preview SDDM Theme: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(Span::styled("   sddm-greeter-qt6 --test-mode --theme /usr/share/sddm/themes/sddm-astronaut-theme/", Style::default().fg(COLOR_DARK))),
        Line::from(""),
        Line::from(vec![
            Span::styled("4. Change Themes Anytime: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("aurora apply-theme <name>", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⚡ System Doctor: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Press [V] to run instant Health Verification Audit", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}

fn render_cheatsheet(f: &mut Frame, _state: &AppState, area: Rect) {
    let block = default_block(" Aurora Keybindings Cheatsheet ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let keybind = |key: &str, action: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("  {:18} ", key), Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(action.to_string(), Style::default().fg(COLOR_WHITE)),
        ])
    };

    let lines = vec![
        Line::from(Span::styled("Primary Desktop Shortcuts (Super = Windows Key):", Style::default().fg(COLOR_MAGENTA))),
        Line::from(""),
        keybind("Super + Return", "Launch Terminal (Kitty / Ghostty)"),
        keybind("Super + Space", "Application Launcher (Rofi)"),
        keybind("Super + E", "Open File Manager (Nemo)"),
        keybind("Super + H", "Aurora Keybindings Guide"),
        keybind("Super + T", "Aurora Theme Switcher GUI"),
        keybind("Super + Q", "Close Focused Window"),
        keybind("Super + F", "Toggle Fullscreen"),
        keybind("Super + M", "Power / Hyprshutdown Menu"),
        keybind("Super + Shift + S", "Screenshot Area (Hyprshot)"),
        keybind("Super + V", "Clipboard History (Cliphist)"),
        keybind("Super + 1..9", "Switch to Workspace 1..9"),
        keybind("Super + Shift + 1..9", "Move Window to Workspace 1..9"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Controls: ", Style::default().fg(COLOR_DARK)),
            Span::styled("[ Enter ] Exit Installer    [ R ] Reboot System", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}
