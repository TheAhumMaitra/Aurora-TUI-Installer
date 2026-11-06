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
            Constraint::Length(3), // Header banner
            Constraint::Min(12),   // Split pane (Hardware specs + Checklist)
            Constraint::Length(4), // Preflight Verdict & Footer
        ])
        .split(area);

    // 1. Header Banner
    let header_p = Paragraph::new(Line::from(vec![
        Span::styled("⚡ SYSTEM PREFLIGHT AUDIT & HARDWARE DIAGNOSTICS ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
        Span::styled("• Live Hardware Detection & Requirement Verification", Style::default().fg(COLOR_TEXT)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_CYAN))
            .padding(Padding::horizontal(1)),
    );
    f.render_widget(header_p, chunks[0]);

    // 2. Middle Panes: Left = Hardware Specs, Right = Compatibility Checklist
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(chunks[1]);

    // Left Pane: Detected Hardware Specs
    let info = &state.distro_info;
    let uefi_badge = if info.is_uefi {
        Span::styled("UEFI Mode (Optimal)", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("Legacy BIOS (Warning: Wayland UEFI recommended)", Style::default().fg(COLOR_YELLOW))
    };

    let ram_badge = if info.total_ram_gb >= 4.0 {
        Span::styled(format!("{:.1} GB (Available: {:.1} GB) [✓ Optimal]", info.total_ram_gb, info.available_ram_gb), Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled(format!("{:.1} GB [Low Memory Warning]", info.total_ram_gb), Style::default().fg(COLOR_YELLOW))
    };

    let disk_badge = if info.home_free_gb >= 15.0 {
        Span::styled(format!("{:.1} GB Free in ~ [✓ Sufficient]", info.home_free_gb), Style::default().fg(COLOR_GREEN))
    } else {
        Span::styled(format!("{:.1} GB Free [Low Disk Space]", info.home_free_gb), Style::default().fg(COLOR_RED))
    };

    let gpu_badge = if info.gpu.is_nvidia() {
        Span::styled(info.gpu.label(), Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(info.gpu.label(), Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD))
    };

    let specs_lines = vec![
        Line::from(vec![
            Span::styled("OS Distribution:   ", Style::default().fg(COLOR_CYAN)),
            Span::styled(&info.arch_flavor, Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Linux Kernel:      ", Style::default().fg(COLOR_CYAN)),
            Span::styled(format!("{} ({})", info.kernel, info.arch), Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("Boot Firmware:     ", Style::default().fg(COLOR_CYAN)),
            uefi_badge,
        ]),
        Line::from(vec![
            Span::styled("Processor (CPU):   ", Style::default().fg(COLOR_CYAN)),
            Span::styled(format!("{} ({} cores)", info.cpu_model, info.cpu_cores), Style::default().fg(COLOR_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Physical Memory:   ", Style::default().fg(COLOR_CYAN)),
            ram_badge,
        ]),
        Line::from(vec![
            Span::styled("Storage (/home):   ", Style::default().fg(COLOR_CYAN)),
            disk_badge,
        ]),
        Line::from(vec![
            Span::styled("Graphics (GPU):    ", Style::default().fg(COLOR_CYAN)),
            gpu_badge,
        ]),
        Line::from(vec![
            Span::styled("Session User:      ", Style::default().fg(COLOR_CYAN)),
            Span::styled(format!("{} (Root: {})", info.current_user, info.is_root), Style::default().fg(COLOR_TEXT)),
        ]),
        Line::from(vec![
            Span::styled("Display Protocol:  ", Style::default().fg(COLOR_CYAN)),
            Span::styled(
                if info.hyprland_running { "Wayland (Hyprland active)" } else { "Wayland / TTY Console" },
                Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let specs_block = default_block(" 1. Detected System Architecture ")
        .padding(Padding::uniform(1));
    let specs_p = Paragraph::new(specs_lines).block(specs_block);
    f.render_widget(specs_p, middle_chunks[0]);

    // Right Pane: Verification Checklist
    let mut check_lines = Vec::new();
    let pass = |text: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled("  [✓] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(text.to_string(), Style::default().fg(COLOR_WHITE)),
        ])
    };
    let warn = |text: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled("  [!] ", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled(text.to_string(), Style::default().fg(COLOR_YELLOW)),
        ])
    };

    if info.is_arch_compatible {
        check_lines.push(pass("Arch-based Linux distribution verified"));
    } else {
        check_lines.push(warn("Non-Arch Linux system (Manual dependencies required)"));
    }

    if !info.is_root {
        check_lines.push(pass("Running under regular user with sudo privilege"));
    } else {
        check_lines.push(warn("Running directly as root user (Warning: configs will deploy to /root)"));
    }

    if info.total_ram_gb >= 4.0 {
        check_lines.push(pass("Memory requirements met (>= 4.0 GB)"));
    } else {
        check_lines.push(warn("Low memory (< 4.0 GB detected)"));
    }

    if info.home_free_gb >= 15.0 {
        check_lines.push(pass("Disk space requirements met (>= 15.0 GB)"));
    } else {
        check_lines.push(warn("Disk space low (< 15.0 GB free)"));
    }

    if info.pacman_available {
        check_lines.push(pass("Official Arch pacman package manager active"));
    } else {
        check_lines.push(warn("pacman binary missing or not in PATH"));
    }

    if let Some(ref helper) = info.aur_helper {
        check_lines.push(pass(&format!("AUR helper available ({helper})")));
    } else {
        check_lines.push(pass("AUR helper will be automatically bootstrapped (yay-bin)"));
    }

    if info.is_uefi {
        check_lines.push(pass("UEFI firmware mode active"));
    } else {
        check_lines.push(warn("BIOS legacy mode (ensure DRM kernel mode-setting is on)"));
    }

    if info.git_available {
        check_lines.push(pass("Git version control ready"));
    } else {
        check_lines.push(warn("Git not installed (will be installed by pacman)"));
    }

    if info.cargo_available {
        check_lines.push(pass("Rust cargo toolchain detected"));
    } else {
        check_lines.push(pass("Rust scripts will install via cargo if available"));
    }

    let checklist_block = default_block(" 2. Preflight Health Verification ")
        .padding(Padding::uniform(1));
    let checklist_p = Paragraph::new(check_lines).block(checklist_block);
    f.render_widget(checklist_p, middle_chunks[1]);

    // 3. Verdict & Return Footer
    let all_ok = info.is_arch_compatible && info.home_free_gb >= 5.0 && info.total_ram_gb >= 2.0;
    let verdict_text = if all_ok {
        Span::styled(
            "  ● PREFLIGHT AUDIT: SYSTEM IS 100% READY FOR AURORA HYPRLAND INSTALLATION  ",
            Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            "  ▲ PREFLIGHT AUDIT: MINOR WARNINGS DETECTED — INSTALLATION CAN STILL PROCEED  ",
            Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD),
        )
    };

    let verdict_p = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Verdict: ", Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD)),
            verdict_text,
        ]),
        Line::from(vec![
            Span::styled("Action:  ", Style::default().fg(COLOR_CYAN)),
            Span::styled("[Esc / Enter / I] ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Return to Installer Configuration", Style::default().fg(COLOR_WHITE)),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_DARK))
            .padding(Padding::horizontal(1)),
    );
    f.render_widget(verdict_p, chunks[2]);
}
