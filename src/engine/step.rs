// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Running,
    Succeeded,
    Warning(String),
    Failed(String),
    Skipped(String),
}

#[allow(dead_code)]
impl StepStatus {
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            StepStatus::Succeeded | StepStatus::Warning(_) | StepStatus::Failed(_) | StepStatus::Skipped(_)
        )
    }

    pub fn is_success(&self) -> bool {
        matches!(self, StepStatus::Succeeded)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, StepStatus::Failed(_))
    }

    pub fn badge(&self) -> &'static str {
        match self {
            StepStatus::Pending => "○",
            StepStatus::Running => "◉",
            StepStatus::Succeeded => "✓",
            StepStatus::Warning(_) => "▲",
            StepStatus::Failed(_) => "✗",
            StepStatus::Skipped(_) => "⊘",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: StepStatus,
    pub duration_secs: Option<f64>,
}

impl InstallStep {
    pub fn new(id: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            status: StepStatus::Pending,
            duration_secs: None,
        }
    }
}
