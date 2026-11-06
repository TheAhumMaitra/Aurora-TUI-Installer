// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::{arch, fedora, nixos, DistroFamily, DistroInfo};
use crate::engine::runner::{EngineEvent, LogLevel};
use crate::engine::step::InstallStep;
use crate::model::config::InstallConfig;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Welcome,
    Configuration,
    PackageSelection,
    Summary,
    Execution,
    Complete,
}

impl AppScreen {
    pub fn index(&self) -> usize {
        match self {
            AppScreen::Welcome => 0,
            AppScreen::Configuration => 1,
            AppScreen::PackageSelection => 2,
            AppScreen::Summary => 3,
            AppScreen::Execution => 4,
            AppScreen::Complete => 5,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            AppScreen::Welcome => "1. Welcome & Distro",
            AppScreen::Configuration => "2. Configuration",
            AppScreen::PackageSelection => "2b. Packages",
            AppScreen::Summary => "3. Review",
            AppScreen::Execution => "4. Installation",
            AppScreen::Complete => "5. Finished",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSection {
    Preset,
    Mode,
    Neovim,
    Terminal,
    Components,
    Theme,
    DryRun,
}

pub struct AppState {
    pub current_screen: AppScreen,
    pub distro_info: DistroInfo,
    pub selected_distro: DistroFamily,
    pub config: InstallConfig,
    pub steps: Vec<InstallStep>,
    pub current_step_index: usize,
    pub overall_progress: u16,
    pub logs: Vec<(LogLevel, String, String)>, // (level, timestamp, text)
    pub log_scroll: usize,
    pub auto_scroll_logs: bool,
    pub active_config_section: ConfigSection,
    pub component_cursor: usize,
    pub theme_cursor: usize,
    pub package_category_cursor: usize,
    pub package_item_cursor: usize,
    pub package_side_packages: bool,
    pub is_installing: bool,
    pub install_finished: bool,
    pub install_success: bool,
    pub summary_message: String,
    pub spinner_tick: usize,
    pub should_exit: bool,
    pub tx_events: Sender<EngineEvent>,
    pub rx_events: Receiver<EngineEvent>,
}

impl AppState {
    pub fn new(distro_info: DistroInfo, initial_config: InstallConfig) -> Self {
        let (tx_events, rx_events) = channel();
        let selected_distro = distro_info.family;

        let mut app = Self {
            current_screen: AppScreen::Welcome,
            distro_info,
            selected_distro,
            config: initial_config,
            steps: Vec::new(),
            current_step_index: 0,
            overall_progress: 0,
            logs: Vec::new(),
            log_scroll: 0,
            auto_scroll_logs: true,
            active_config_section: ConfigSection::Preset,
            component_cursor: 0,
            theme_cursor: 0,
            package_category_cursor: 0,
            package_item_cursor: 0,
            package_side_packages: false,
            is_installing: false,
            install_finished: false,
            install_success: false,
            summary_message: String::new(),
            spinner_tick: 0,
            should_exit: false,
            tx_events,
            rx_events,
        };

        app.refresh_steps();
        app
    }

    pub fn refresh_steps(&mut self) {
        self.steps = match self.selected_distro {
            DistroFamily::Arch => arch::build_steps(&self.config),
            DistroFamily::Fedora => fedora::build_steps(&self.config),
            DistroFamily::NixOS => nixos::build_steps(&self.config),
            DistroFamily::Unknown => Vec::new(),
        };
    }

    pub fn set_distro(&mut self, distro: DistroFamily) {
        self.selected_distro = distro;
        self.refresh_steps();
    }

    pub fn cycle_distro(&mut self) {
        let next = match self.selected_distro {
            DistroFamily::Arch => DistroFamily::Fedora,
            DistroFamily::Fedora => DistroFamily::NixOS,
            DistroFamily::NixOS => DistroFamily::Arch,
            DistroFamily::Unknown => DistroFamily::Arch,
        };
        self.set_distro(next);
    }

    pub fn start_installation(&mut self) {
        if self.is_installing || self.install_finished {
            return;
        }

        self.is_installing = true;
        self.current_screen = AppScreen::Execution;
        self.refresh_steps();

        let tx = self.tx_events.clone();
        let distro = self.selected_distro;
        let config = self.config.clone();

        std::thread::spawn(move || {
            crate::engine::execute_installation(distro, config, tx);
        });
    }

    pub fn poll_events(&mut self) {
        while let Ok(event) = self.rx_events.try_recv() {
            match event {
                EngineEvent::StepStarted { index, .. } => {
                    self.current_step_index = index;
                    if let Some(step) = self.steps.get_mut(index) {
                        step.status = crate::engine::StepStatus::Running;
                    }
                }
                EngineEvent::StepCompleted {
                    index,
                    status,
                    duration_secs,
                    ..
                } => {
                    if let Some(step) = self.steps.get_mut(index) {
                        step.status = status;
                        step.duration_secs = Some(duration_secs);
                    }
                }
                EngineEvent::LogLine { level, text } => {
                    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
                    self.logs.push((level, timestamp, text));
                    if self.auto_scroll_logs && self.logs.len() > 10 {
                        self.log_scroll = self.logs.len().saturating_sub(1);
                    }
                }
                EngineEvent::Progress { percent } => {
                    self.overall_progress = percent;
                }
                EngineEvent::Finished {
                    success,
                    summary,
                    ..
                } => {
                    self.is_installing = false;
                    self.install_finished = true;
                    self.install_success = success;
                    self.summary_message = summary;
                    self.overall_progress = 100;
                    self.current_screen = AppScreen::Complete;
                }
            }
        }
    }

    pub fn tick_spinner(&mut self) {
        self.spinner_tick = (self.spinner_tick + 1) % 10;
    }

    pub fn spinner_char(&self) -> char {
        const SPINNER: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        SPINNER[self.spinner_tick % SPINNER.len()]
    }
}
