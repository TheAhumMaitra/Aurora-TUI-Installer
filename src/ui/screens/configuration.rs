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

    // Left pane: Presets, Neovim, Themes & Swatches, Flags
    render_left_pane(f, state, chunks[0]);

    // Right pane: Components Checklist, Package Shortcut & Details
    render_right_pane(f, state, chunks[1]);
}

fn render_left_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // 1. Preset & Package Shortcuts
            Constraint::Length(5), // 2. Developer Suite & Web Core
            Constraint::Length(7), // 3. Rice Aesthetics (Bar, Launcher, DM, Motion, Lock, Font)
            Constraint::Min(6),    // 4. Theme picker with live palette swatches
            Constraint::Length(5), // 5. Audio, Kernel & Flags
        ])
        .split(area);

    // 1. Preset Box & Package Shortcuts
    let preset_active = state.active_config_section == ConfigSection::Preset;
    let preset_block = if preset_active {
        active_block(" 1. Preset Profile [P to cycle, K for Packages, A for All, X for None] ")
    } else {
        default_block(" 1. Preset Profile ")
    };

    let all_pkgs = crate::distro::arch::all_arch_packages();
    let total_pkgs = all_pkgs.len();
    let disabled_count = state.config.disabled_packages.len();
    let enabled_count = total_pkgs.saturating_sub(disabled_count);

    let pkg_tag = if disabled_count > 0 {
        Span::styled(format!(" [{}/{} Active] ", enabled_count, total_pkgs), Style::default().fg(COLOR_YELLOW))
    } else {
        Span::styled(format!(" [{}/{} Active] ", total_pkgs, total_pkgs), Style::default().fg(COLOR_GREEN))
    };

    let preset_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Preset: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.preset.label(),
                Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled("[K] Package Selector", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            pkg_tag,
        ]),
        Line::from(vec![
            Span::styled("Custom Actions: ", Style::default().fg(COLOR_DARK)),
            Span::styled("[A] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Select All Packages   ", Style::default().fg(COLOR_WHITE)),
            Span::styled("[X] ", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
            Span::styled("Deselect All Packages (Clean Slate)", Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(Span::styled(
            state.config.preset.description(),
            Style::default().fg(COLOR_DARK),
        )),
    ])
    .block(preset_block.padding(Padding::horizontal(1)));
    f.render_widget(preset_p, chunks[0]);

    // 2. Developer Suite & Web Core (Neovim, Terminal, Browser, Mode)
    let dev_active = state.active_config_section == ConfigSection::Developer;
    let dev_block = if dev_active {
        active_block(" 2. Developer Suite & Web Core [V: Nvim, T: Term, W: Web, M: Channel] ")
    } else {
        default_block(" 2. Developer Suite & Web Core ")
    };

    let dev_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Neovim IDE:  ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.neovim_preset.label(),
                Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [V]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Terminal:    ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.terminal_choice.label(),
                Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [T]", Style::default().fg(COLOR_DARK)),
            Span::raw("   "),
            Span::styled("Channel: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(state.config.mode.label(), Style::default().fg(COLOR_YELLOW)),
            Span::styled(" [M]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Web Browser: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.browser_choice.label(),
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [W]", Style::default().fg(COLOR_DARK)),
        ]),
    ])
    .block(dev_block.padding(Padding::horizontal(1)));
    f.render_widget(dev_p, chunks[1]);

    // 3. Desktop Rice Aesthetics (Waybar, Rofi, Display Manager, Lockscreen, Font, Motion)
    let aes_active = state.active_config_section == ConfigSection::Aesthetics;
    let aes_block = if aes_active {
        active_block(" 3. Desktop Rice Aesthetics [B: Bar, R: Rofi, C: Lock, F: Font, L: DM, O: Anim] ")
    } else {
        default_block(" 3. Desktop Rice Aesthetics ")
    };

    let aes_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Waybar:      ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.bar_layout.short_name(),
                Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [B]", Style::default().fg(COLOR_DARK)),
            Span::raw("   "),
            Span::styled("Launcher: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.launcher_style.short_name(),
                Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [R]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Lockscreen:  ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.lockscreen.label(),
                Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [C]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Mono Font:   ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.font_choice.label(),
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [F]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Display Mgr: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                state.config.display_manager.label(),
                Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [L]", Style::default().fg(COLOR_DARK)),
            Span::raw("   "),
            Span::styled("Motion: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(state.config.animation_profile.label(), Style::default().fg(COLOR_GREEN)),
            Span::styled(" [O]", Style::default().fg(COLOR_DARK)),
        ]),
    ])
    .block(aes_block.padding(Padding::horizontal(1)));
    f.render_widget(aes_p, chunks[2]);

    // 4. Theme Selector with Live Color Swatches
    let theme_active = state.active_config_section == ConfigSection::Theme;
    let theme_block = if theme_active {
        active_block(" 4. Desktop Theme & Palette [↑/↓ to choose, Space to select] ")
    } else {
        default_block(" 4. Desktop Theme & Palette ")
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

        // Live color palette swatches
        let colors = theme.palette_colors();
        let swatch1 = Span::styled("██", Style::default().fg(colors[0]));
        let swatch2 = Span::styled("██", Style::default().fg(colors[1]));
        let swatch3 = Span::styled("██", Style::default().fg(colors[2]));
        let swatch4 = Span::styled("██ ", Style::default().fg(colors[3]));

        theme_lines.push(Line::from(vec![
            Span::styled(pointer, Style::default().fg(COLOR_MAGENTA)),
            Span::styled(marker, Style::default().fg(COLOR_CYAN)),
            swatch1,
            swatch2,
            swatch3,
            swatch4,
            Span::styled(theme.name, style),
        ]));
    }

    let theme_p = Paragraph::new(theme_lines)
        .block(theme_block.padding(Padding::horizontal(1)));
    f.render_widget(theme_p, chunks[3]);

    // 5. Audio, Performance & System Flags
    let flags_active = state.active_config_section == ConfigSection::SystemFlags;
    let flags_block = if flags_active {
        active_block(" 5. Audio & Performance Flags [E: Sound, Z: Kernel, H: Shell, G, D, S] ")
    } else {
        default_block(" 5. Audio & Performance Flags ")
    };

    let dry_box = if state.config.dry_run {
        Span::styled("[X] Dry-Run Mode [D]  ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("[ ] Dry-Run Mode [D]  ", Style::default().fg(COLOR_TEXT))
    };

    let sudo_box = if state.config.allow_sudo_rs {
        Span::styled("[X] Sudo-rs Memory Safe [S]  ", Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled("[ ] Sudo-rs Memory Safe [S]  ", Style::default().fg(COLOR_DARK))
    };

    let gaming_box = if state.config.is_component_selected(Component::GamingSuite) {
        Span::styled("[X] Gaming Suite [G] (Steam, GameMode, MangoHud)", Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("[ ] Gaming Suite [G] (Steam, GameMode, MangoHud)", Style::default().fg(COLOR_DARK))
    };

    let flags_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Sound Suite:   ", Style::default().fg(COLOR_CYAN)),
            Span::styled(state.config.audio_suite.label(), Style::default().fg(COLOR_WHITE)),
            Span::styled(" [E]", Style::default().fg(COLOR_DARK)),
            Span::raw("   "),
            Span::styled("Kernel: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(state.config.kernel_tweaks.short_name(), Style::default().fg(COLOR_GREEN)),
            Span::styled(" [Z]", Style::default().fg(COLOR_DARK)),
        ]),
        Line::from(vec![
            Span::styled("Default Shell: ", Style::default().fg(COLOR_CYAN)),
            Span::styled(state.config.shell_choice.label(), Style::default().fg(COLOR_WHITE)),
            Span::styled(" [H]", Style::default().fg(COLOR_DARK)),
            Span::raw("   "),
            gaming_box,
        ]),
        Line::from(vec![dry_box, sudo_box]),
    ])
    .block(flags_block.padding(Padding::horizontal(1)));
    f.render_widget(flags_p, chunks[4]);
}

fn render_right_pane(f: &mut Frame, state: &AppState, area: Rect) {
    let comp_active = state.active_config_section == ConfigSection::Components;
    let block = if comp_active {
        active_block(" Components Checklist [↑/↓ to navigate, Space to toggle, K for Packages] ")
    } else {
        default_block(" Components Checklist [Press K to customize packages] ")
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
            Span::styled("Module: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
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
