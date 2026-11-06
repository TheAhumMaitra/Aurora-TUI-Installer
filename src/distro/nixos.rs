// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::engine::step::InstallStep;
use crate::model::config::{Component, InstallConfig};

pub const NIXOS_CONFIG_FILES: &[&str] = &[
    "configuration.nix",
    "home.nix",
    "hardware-configuration.nix",
    "flake.nix",
    "flake.lock",
    "variables.nix",
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
            "Backup current configurations and deploy Hyprland, Waybar, SwayNC, Kitty configs",
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

    // 8. LazyVim
    if config.is_component_selected(Component::LazyVim) {
        steps.push(InstallStep::new(
            "nixos_lazyvim",
            "Deploy LazyVim Neovim Configuration",
            "Backup existing Neovim configs and deploy modern LazyVim starter setup",
        ));
    }

    // 9. Shell & PATH
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "nixos_shell",
            "Configure Shell & PATH Environment",
            "Ensure ~/.cargo/bin and ~/.local/bin are present in shell PATH profiles",
        ));
    }

    // 10. Theme
    steps.push(InstallStep::new(
        "nixos_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}
