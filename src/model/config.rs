// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallMode {
    Stable,
    Git,
}

impl InstallMode {
    pub fn label(&self) -> &'static str {
        match self {
            InstallMode::Stable => "Stable (Recommended)",
            InstallMode::Git => "Bleeding Edge (Git / Master)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallMode::Stable => "Uses official, well-tested package releases from distributions",
            InstallMode::Git => "Builds or fetches latest git master packages (hyprland-git, etc.)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallPreset {
    FullRice,
    MinimalRice,
    Custom,
}

impl InstallPreset {
    pub fn label(&self) -> &'static str {
        match self {
            InstallPreset::FullRice => "Full Rice (Recommended)",
            InstallPreset::MinimalRice => "Minimal Core",
            InstallPreset::Custom => "Custom Selection",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallPreset::FullRice => "Installs everything: desktop, SDDM astronaut theme, Waytrogen, LazyVim, tools & fonts",
            InstallPreset::MinimalRice => "Installs only core Hyprland, Waybar, basic dotfiles and essential utils",
            InstallPreset::Custom => "Fine-tune and toggle each individual component to your preference",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Component {
    SystemPackages,
    ExtraRepositories,
    SddmTheme,
    RustScripts,
    Waytrogen,
    LazyVim,
    Dotfiles,
    ShellConfig,
    SudoRs,
}

impl Component {
    pub const ALL: &[Component] = &[
        Component::SystemPackages,
        Component::ExtraRepositories,
        Component::SddmTheme,
        Component::RustScripts,
        Component::Waytrogen,
        Component::LazyVim,
        Component::Dotfiles,
        Component::ShellConfig,
        Component::SudoRs,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Component::SystemPackages => "System & Desktop Packages (Hyprland, Waybar, Rofi, Audio)",
            Component::ExtraRepositories => "Package Repositories (AUR / COPR / Flakes & Zen Browser)",
            Component::SddmTheme => "SDDM Astronaut Theme & Font Setup",
            Component::RustScripts => "Aurora Hyprland Rust Scripts & Tools (aurora CLI)",
            Component::Waytrogen => "Waytrogen-Aurora Wallpaper Switcher",
            Component::LazyVim => "LazyVim Modern Neovim Starter",
            Component::Dotfiles => "Aurora Dotfiles (~/.config, preserves User customization)",
            Component::ShellConfig => "Shell PATH & Fish Configuration (~/.cargo/bin, ~/.local/bin)",
            Component::SudoRs => "Switch sudo to sudo-rs (Memory-safe Rust implementation)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Component::SystemPackages => "Installs Hyprland, Waybar, SwayNC, Rofi, Pipewire, Wireplumber, fonts, and terminal",
            Component::ExtraRepositories => "Enables AUR helpers, Fedora COPR repositories, or Nix Flakes for extra ricing tools",
            Component::SddmTheme => "Installs SDDM login manager astronaut theme with elegant animations and fonts",
            Component::RustScripts => "Compiles and installs Aurora's high-performance Rust helper scripts into ~/.cargo/bin",
            Component::Waytrogen => "Integrates the fast GTK/Rust wallpaper daemon and management tool for Wayland",
            Component::LazyVim => "Backs up Neovim configs and deploys the preconfigured LazyVim IDE setup",
            Component::Dotfiles => "Safely deploys Waybar, Hyprland, Kitty, Ghostty, and SwayNC configs with timestamped backups",
            Component::ShellConfig => "Configures environment variables, adds ~/.cargo/bin and ~/.local/bin to PATH across shells",
            Component::SudoRs => "Optionally symlinks /usr/bin/sudo to /usr/bin/sudo-rs for modern Rust memory safety",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallConfig {
    pub mode: InstallMode,
    pub preset: InstallPreset,
    pub selected_components: Vec<Component>,
    pub theme_name: String,
    pub dry_run: bool,
    pub backup_dir: PathBuf,
    pub source_dir: PathBuf,
    pub allow_sudo_rs: bool,
}

impl Default for InstallConfig {
    fn default() -> Self {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let home = dirs_home().unwrap_or_else(|| PathBuf::from("/tmp"));
        let backup_dir = home.join(format!(".config/aurora_backup_{}", timestamp));

        // Default components for FullRice
        let selected_components = vec![
            Component::SystemPackages,
            Component::ExtraRepositories,
            Component::SddmTheme,
            Component::RustScripts,
            Component::Waytrogen,
            Component::LazyVim,
            Component::Dotfiles,
            Component::ShellConfig,
            Component::SudoRs,
        ];

        Self {
            mode: InstallMode::Stable,
            preset: InstallPreset::FullRice,
            selected_components,
            theme_name: "Aurora Default".to_string(),
            dry_run: false,
            backup_dir,
            source_dir: PathBuf::new(),
            allow_sudo_rs: true,
        }
    }
}

impl InstallConfig {
    pub fn is_component_selected(&self, component: Component) -> bool {
        self.selected_components.contains(&component)
    }

    pub fn toggle_component(&mut self, component: Component) {
        if let Some(pos) = self.selected_components.iter().position(|c| *c == component) {
            self.selected_components.remove(pos);
        } else {
            self.selected_components.push(component);
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn apply_preset(&mut self, preset: InstallPreset) {
        self.preset = preset;
        match preset {
            InstallPreset::FullRice => {
                self.selected_components = Component::ALL.to_vec();
            }
            InstallPreset::MinimalRice => {
                self.selected_components = vec![
                    Component::SystemPackages,
                    Component::Dotfiles,
                    Component::RustScripts,
                    Component::ShellConfig,
                ];
            }
            InstallPreset::Custom => {}
        }
    }
}

pub fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
