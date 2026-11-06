// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::engine::runner::LogLevel;
use crate::engine::step::StepStatus;
use crate::model::state::AppState;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Padding, Paragraph};
use ratatui::Frame;

pub fn render(f: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Top Progress Bar & Current Task
            Constraint::Min(10),   // Split: Steps Pipeline (left) & Streaming Logs (right)
        ])
        .split(area);

    render_progress_bar(f, state, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(chunks[1]);

    render_step_pipeline(f, state, body_chunks[0]);
    render_terminal_logs(f, state, body_chunks[1]);
}

fn render_progress_bar(f: &mut Frame, state: &AppState, area: Rect) {
    let current_step = state.steps.get(state.current_step_index);
    let title = if let Some(step) = current_step {
        if state.is_installing {
            format!(" {} Step {}/{}: {} ", state.spinner_char(), state.current_step_index + 1, state.steps.len(), step.title)
        } else if state.install_finished {
            " Installation Complete! ".to_string()
        } else {
            " Initializing Pipeline... ".to_string()
        }
    } else {
        " Preparing Aurora Installation... ".to_string()
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_CYAN)),
        )
        .gauge_style(
            Style::default()
                .fg(COLOR_CYAN)
                .bg(COLOR_DARK)
                .add_modifier(Modifier::BOLD),
        )
        .percent(state.overall_progress)
        .label(format!("{}%", state.overall_progress));

    f.render_widget(gauge, area);
}

fn render_step_pipeline(f: &mut Frame, state: &AppState, area: Rect) {
    let block = default_block(" Pipeline Progress ")
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    for (i, step) in state.steps.iter().enumerate() {
        let (badge, badge_style, text_style) = match &step.status {
            StepStatus::Pending => (
                "○ ",
                Style::default().fg(COLOR_DARK),
                Style::default().fg(COLOR_DARK),
            ),
            StepStatus::Running => (
                "▶ ",
                Style::default().fg(COLOR_MAGENTA).add_modifier(Modifier::BOLD),
                Style::default().fg(COLOR_WHITE).add_modifier(Modifier::BOLD),
            ),
            StepStatus::Succeeded => (
                "✓ ",
                Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD),
                Style::default().fg(COLOR_TEXT),
            ),
            StepStatus::Warning(_) => (
                "▲ ",
                Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD),
                Style::default().fg(COLOR_YELLOW),
            ),
            StepStatus::Failed(_) => (
                "✗ ",
                Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
                Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
            ),
            StepStatus::Skipped(_) => (
                "⊘ ",
                Style::default().fg(COLOR_DARK),
                Style::default().fg(COLOR_DARK),
            ),
        };

        let duration_str = if let Some(secs) = step.duration_secs {
            format!(" ({:.1}s)", secs)
        } else if step.status == StepStatus::Running {
            format!(" ({} running)", state.spinner_char())
        } else {
            String::new()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{:2}. ", i + 1), Style::default().fg(COLOR_DARK)),
            Span::styled(badge, badge_style),
            Span::styled(&step.title, text_style),
            Span::styled(duration_str, Style::default().fg(COLOR_CYAN)),
        ]));
    }

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}

fn render_terminal_logs(f: &mut Frame, state: &AppState, area: Rect) {
    let autoscroll_tag = if state.auto_scroll_logs {
        "[Auto-scroll ON: S]"
    } else {
        "[Auto-scroll OFF: S]"
    };
    let title = format!(" Live Terminal Logs ({} entries) {} ", state.logs.len(), autoscroll_tag);

    let block = active_block(title)
        .padding(Padding::uniform(1));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    for (level, time, text) in &state.logs {
        let (tag, tag_style) = match level {
            LogLevel::Info => ("INFO", Style::default().fg(COLOR_CYAN)),
            LogLevel::Success => (" OK ", Style::default().fg(COLOR_GREEN).add_modifier(Modifier::BOLD)),
            LogLevel::Warn => ("WARN", Style::default().fg(COLOR_YELLOW).add_modifier(Modifier::BOLD)),
            LogLevel::Error => ("FAIL", Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD)),
            LogLevel::Debug => ("DBUG", Style::default().fg(COLOR_DARK)),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", time), Style::default().fg(COLOR_DARK)),
            Span::styled(format!("[{}] ", tag), tag_style),
            Span::styled(text, Style::default().fg(COLOR_TEXT)),
        ]));
    }

    // Scroll handling: show latest lines if autoscroll is enabled
    let height = inner.height as usize;
    let scroll_y = if state.auto_scroll_logs && lines.len() > height {
        (lines.len() - height) as u16
    } else {
        state.log_scroll.min(lines.len().saturating_sub(height)) as u16
    };

    let p = Paragraph::new(lines).scroll((scroll_y, 0));
    f.render_widget(p, inner);
}
