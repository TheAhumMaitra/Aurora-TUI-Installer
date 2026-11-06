// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallMode {
    Stable,
    Git,
}

#[allow(dead_code)]
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
            InstallPreset::FullRice => "Installs everything: desktop, SDDM astronaut theme, Waytrogen, Neovim, tools & fonts",
            InstallPreset::MinimalRice => "Installs only core Hyprland, Waybar, basic dotfiles and essential utils",
            InstallPreset::Custom => "Fine-tune individual components, package lists, and development presets",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeovimPreset {
    LazyVim,
    NvChad,
    AstroNvim,
    Kickstart,
    None,
}

#[allow(dead_code)]
impl NeovimPreset {
    pub const ALL: &[NeovimPreset] = &[
        NeovimPreset::LazyVim,
        NeovimPreset::NvChad,
        NeovimPreset::AstroNvim,
        NeovimPreset::Kickstart,
        NeovimPreset::None,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            NeovimPreset::LazyVim => "LazyVim Starter (Aurora Official)",
            NeovimPreset::NvChad => "NvChad (Blazing Fast & Beautiful)",
            NeovimPreset::AstroNvim => "AstroNvim (Aesthetic Extensible IDE)",
            NeovimPreset::Kickstart => "Kickstart.nvim (Modular & Minimal)",
            NeovimPreset::None => "None / Keep Existing Configuration",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            NeovimPreset::LazyVim => "Preconfigured Neovim with Lazy.nvim, LSP, treesitter, fuzzy finder & which-key",
            NeovimPreset::NvChad => "Blazing fast startup time (<20ms) with rich UI popups and custom statusline",
            NeovimPreset::AstroNvim => "Feature-rich extensible configuration tailored for modern full-stack development",
            NeovimPreset::Kickstart => "Single-file modular starting point documented for personal customization",
            NeovimPreset::None => "Do not touch ~/.config/nvim; preserve current user Neovim setup",
        }
    }

    pub fn repo_url(&self) -> Option<&'static str> {
        match self {
            NeovimPreset::LazyVim => Some("https://github.com/LazyVim/starter"),
            NeovimPreset::NvChad => Some("https://github.com/NvChad/starter"),
            NeovimPreset::AstroNvim => Some("https://github.com/AstroNvim/template"),
            NeovimPreset::Kickstart => Some("https://github.com/nvim-lua/kickstart.nvim"),
            NeovimPreset::None => None,
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            NeovimPreset::LazyVim => NeovimPreset::NvChad,
            NeovimPreset::NvChad => NeovimPreset::AstroNvim,
            NeovimPreset::AstroNvim => NeovimPreset::Kickstart,
            NeovimPreset::Kickstart => NeovimPreset::None,
            NeovimPreset::None => NeovimPreset::LazyVim,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalChoice {
    Kitty,
    Ghostty,
    Alacritty,
    Foot,
}

#[allow(dead_code)]
impl TerminalChoice {
    pub const ALL: &[TerminalChoice] = &[
        TerminalChoice::Kitty,
        TerminalChoice::Ghostty,
        TerminalChoice::Alacritty,
        TerminalChoice::Foot,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            TerminalChoice::Kitty => "Kitty (Aurora Default)",
            TerminalChoice::Ghostty => "Ghostty (GPU Accelerated)",
            TerminalChoice::Alacritty => "Alacritty (Simplicity & Speed)",
            TerminalChoice::Foot => "Foot (Lightweight Wayland Native)",
        }
    }

    pub fn package_name(&self) -> &'static str {
        match self {
            TerminalChoice::Kitty => "kitty",
            TerminalChoice::Ghostty => "ghostty",
            TerminalChoice::Alacritty => "alacritty",
            TerminalChoice::Foot => "foot",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            TerminalChoice::Kitty => TerminalChoice::Ghostty,
            TerminalChoice::Ghostty => TerminalChoice::Alacritty,
            TerminalChoice::Alacritty => TerminalChoice::Foot,
            TerminalChoice::Foot => TerminalChoice::Kitty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellChoice {
    Fish,
    Zsh,
    Bash,
}

#[allow(dead_code)]
impl ShellChoice {
    pub const ALL: &[ShellChoice] = &[ShellChoice::Fish, ShellChoice::Zsh, ShellChoice::Bash];

    pub fn label(&self) -> &'static str {
        match self {
            ShellChoice::Fish => "Fish (Smart Autosuggestions)",
            ShellChoice::Zsh => "Zsh + Starship Prompt",
            ShellChoice::Bash => "Bash + Starship Prompt",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            ShellChoice::Fish => ShellChoice::Zsh,
            ShellChoice::Zsh => ShellChoice::Bash,
            ShellChoice::Bash => ShellChoice::Fish,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationProfile {
    Snappy,
    Smooth,
    Minimal,
}

impl AnimationProfile {
    pub fn label(&self) -> &'static str {
        match self {
            AnimationProfile::Snappy => "Snappy (0.15s - Instant Response)",
            AnimationProfile::Smooth => "Smooth (0.3s - Elegant Curves)",
            AnimationProfile::Minimal => "Minimal (Reduced Motion)",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            AnimationProfile::Snappy => AnimationProfile::Smooth,
            AnimationProfile::Smooth => AnimationProfile::Minimal,
            AnimationProfile::Minimal => AnimationProfile::Snappy,
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
    GamingSuite,
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
        Component::GamingSuite,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Component::SystemPackages => "System & Desktop Packages (Hyprland, Waybar, Rofi, Audio)",
            Component::ExtraRepositories => "Package Repositories (AUR / COPR / Flakes & Zen Browser)",
            Component::SddmTheme => "SDDM Astronaut Theme & Font Setup",
            Component::RustScripts => "Aurora Hyprland Rust Scripts & Tools (aurora CLI)",
            Component::Waytrogen => "Waytrogen-Aurora Wallpaper Switcher",
            Component::LazyVim => "Neovim IDE Configuration Setup",
            Component::Dotfiles => "Aurora Dotfiles (~/.config, preserves User customization)",
            Component::ShellConfig => "Shell PATH & Default Shell Configuration",
            Component::SudoRs => "Switch sudo to sudo-rs (Memory-safe Rust implementation)",
            Component::GamingSuite => "Gaming & Performance Suite (Steam, GameMode, MangoHud)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Component::SystemPackages => "Installs Hyprland, Waybar, SwayNC, Rofi, Pipewire, Wireplumber, fonts, and terminal",
            Component::ExtraRepositories => "Enables AUR helpers, Fedora COPR repositories, or Nix Flakes for extra ricing tools",
            Component::SddmTheme => "Installs SDDM login manager astronaut theme with elegant animations and fonts",
            Component::RustScripts => "Compiles and installs Aurora's high-performance Rust helper scripts into ~/.cargo/bin",
            Component::Waytrogen => "Integrates the fast GTK/Rust wallpaper daemon and management tool for Wayland",
            Component::LazyVim => "Deploys selected Neovim IDE preset (LazyVim, NvChad, AstroNvim, Kickstart)",
            Component::Dotfiles => "Safely deploys Waybar, Hyprland, Kitty, Ghostty, and SwayNC configs with timestamped backups",
            Component::ShellConfig => "Configures environment variables, default shell, and adds ~/.cargo/bin and ~/.local/bin to PATH",
            Component::SudoRs => "Optionally symlinks /usr/bin/sudo to /usr/bin/sudo-rs for modern Rust memory safety",
            Component::GamingSuite => "Installs Steam, GameMode optimizer daemon, MangoHud FPS overlay, and gaming tweaks",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallConfig {
    pub mode: InstallMode,
    pub preset: InstallPreset,
    pub selected_components: Vec<Component>,
    pub neovim_preset: NeovimPreset,
    pub terminal_choice: TerminalChoice,
    pub shell_choice: ShellChoice,
    pub animation_profile: AnimationProfile,
    pub theme_name: String,
    pub dry_run: bool,
    pub backup_dir: PathBuf,
    pub source_dir: PathBuf,
    pub allow_sudo_rs: bool,
    pub disabled_packages: HashSet<String>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let home = dirs_home().unwrap_or_else(|| PathBuf::from("/tmp"));
        let backup_dir = home.join(format!(".config/aurora_backup_{}", timestamp));

        // Default components for FullRice (GamingSuite optional by default)
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
            neovim_preset: NeovimPreset::LazyVim,
            terminal_choice: TerminalChoice::Kitty,
            shell_choice: ShellChoice::Fish,
            animation_profile: AnimationProfile::Snappy,
            theme_name: "Aurora Default".to_string(),
            dry_run: false,
            backup_dir,
            source_dir: PathBuf::new(),
            allow_sudo_rs: true,
            disabled_packages: HashSet::new(),
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
                self.selected_components = vec![
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
                self.disabled_packages.clear();
            }
            InstallPreset::MinimalRice => {
                self.selected_components = vec![
                    Component::SystemPackages,
                    Component::Dotfiles,
                    Component::RustScripts,
                    Component::ShellConfig,
                ];
                self.neovim_preset = NeovimPreset::None;
                self.disabled_packages.clear();
            }
            InstallPreset::Custom => {}
        }
    }

    pub fn is_package_enabled(&self, package_name: &str) -> bool {
        !self.disabled_packages.contains(package_name)
    }

    pub fn toggle_package(&mut self, package_name: &str) {
        if self.disabled_packages.contains(package_name) {
            self.disabled_packages.remove(package_name);
        } else {
            self.disabled_packages.insert(package_name.to_string());
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn enable_all_packages<'a>(&mut self, pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in pkgs {
            self.disabled_packages.remove(pkg);
        }
    }

    pub fn disable_all_packages<'a>(&mut self, pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in pkgs {
            self.disabled_packages.insert(pkg.to_string());
        }
        self.preset = InstallPreset::Custom;
    }
}

pub fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
