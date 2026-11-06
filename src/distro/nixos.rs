// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code)]

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{Component, InstallConfig, NeovimPreset};

pub const NIXOS_CONFIG_FILES: &[&str] = &[
    "configuration.nix",
    "home.nix",
    "hardware-configuration.nix",
    "flake.nix",
    "flake.lock",
    "variables.nix",
];

pub const NIXOS_CORE_PKGS: &[&str] = &[
    "hyprland",
    "xdg-desktop-portal-hyprland",
    "pipewire",
    "wireplumber",
    "networkmanager",
];

pub const NIXOS_UI_PKGS: &[&str] = &[
    "waybar",
    "rofi",
    "swaynotificationcenter",
    "hyprlock",
    "hypridle",
];

pub const NIXOS_UTILS_PKGS: &[&str] = &[
    "kitty",
    "ghostty",
    "starship",
    "neovim",
    "cliphist",
    "wl-clipboard",
    "mise",
];

pub const NIXOS_EXTRAS_PKGS: &[&str] = &[
    "app.zen_browser.zen",
    "nordzy-cursors",
    "sddm-astronaut-theme",
];

pub const NIXOS_GAMING_PKGS: &[&str] = &[
    "steam",
    "gamemode",
    "mangohud",
];

pub const NIXOS_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "Core Compositor (Flake)",
        description: "Hyprland compositor, XDG portal, audio services",
        packages: NIXOS_CORE_PKGS,
    },
    DistroPackageGroup {
        category: "UI & Notification Daemons",
        description: "Waybar status bar, Rofi, SwayNC, Hyprlock",
        packages: NIXOS_UI_PKGS,
    },
    DistroPackageGroup {
        category: "Utilities & Terminal Suite",
        description: "Kitty, Ghostty, Starship, Neovim, Mise, Cliphist",
        packages: NIXOS_UTILS_PKGS,
    },
    DistroPackageGroup {
        category: "Rice Extras & Theme Assets",
        description: "Zen Browser Flatpak, Nordzy cursors, SDDM astronaut theme",
        packages: NIXOS_EXTRAS_PKGS,
    },
    DistroPackageGroup {
        category: "Gaming & Performance Suite",
        description: "Steam gaming client, GameMode optimization daemon, MangoHud",
        packages: NIXOS_GAMING_PKGS,
    },
];

pub fn build_steps(config: &InstallConfig) -> Vec<InstallStep> {
    let mut steps = Vec::new();

    // 1. Preflight
    steps.push(InstallStep::new(
        "nixos_preflight",
        "System Preflight & Nix Flakes Check",
        "Verify NixOS environment, non-root user privileges, and enable nix flakes feature",
    ));

    // 2. NixOS Configuration & System Rebuild
    if config.is_component_selected(Component::SystemPackages) {
        steps.push(InstallStep::new(
            "nixos_copy_config",
            "Deploy NixOS Flake Configuration",
            "Backup /etc/nixos and deploy configuration.nix, home.nix, and flake.nix",
        ));

        steps.push(InstallStep::new(
            "nixos_rebuild_switch",
            "NixOS System Rebuild & Switch",
            "Execute 'nixos-rebuild switch --flake /etc/nixos#nixos' to build system generation",
        ));
    }

    // 3. Dotfiles
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "nixos_dotfiles",
            "Deploy Aurora Hyprland Dotfiles",
            format!("Deploy Hyprland (Animation: {}), Waybar, SwayNC, Kitty configs", config.animation_profile.label()),
        ));
    }

    // 4. Rust scripts & packages
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "nixos_rust_scripts",
            "Build Aurora Scripts in Nix Dev Shell",
            "Build aurora CLI helper scripts, termflix, and nmrs-tui inside Nix develop shell",
        ));
    }

    // 5. SDDM Astronaut Theme
    if config.is_component_selected(Component::SddmTheme) {
        steps.push(InstallStep::new(
            "nixos_sddm",
            "Configure SDDM Astronaut Theme",
            "Clone and configure astronaut theme assets and fonts for SDDM greeter",
        ));
    }

    // 6. Waytrogen
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "nixos_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone and build waytrogen-aurora wallpaper manager inside dev shell",
        ));
    }

    // 7. Extra tools (Mise, Nordzy cursors, Zen Browser)
    if config.is_component_selected(Component::ExtraRepositories) {
        steps.push(InstallStep::new(
            "nixos_extras",
            "Install Mise, Nordzy Cursors & Zen Browser",
            "Install mise runtime manager, Nordzy hyprcursors, and Zen Browser via Flatpak",
        ));
    }

    // 8. Gaming Suite
    if config.is_component_selected(Component::GamingSuite) {
        steps.push(InstallStep::new(
            "nixos_gaming",
            "Install Gaming & Performance Suite",
            "Configure Steam client and GameMode optimization daemon on NixOS",
        ));
    }

    // 9. Neovim Setup
    if config.is_component_selected(Component::LazyVim) && config.neovim_preset != NeovimPreset::None {
        steps.push(InstallStep::new(
            "nixos_lazyvim",
            format!("Deploy {} Setup", config.neovim_preset.label()),
            format!("Backup existing Neovim configs and deploy {}", config.neovim_preset.description()),
        ));
    }

    // 10. Shell & PATH
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "nixos_shell",
            format!("Configure Shell ({}) & PATH Environment", config.shell_choice.label()),
            "Ensure ~/.cargo/bin and ~/.local/bin are present in shell PATH profiles",
        ));
    }

    // 11. Theme
    steps.push(InstallStep::new(
        "nixos_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}
