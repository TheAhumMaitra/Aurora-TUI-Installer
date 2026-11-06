// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::engine::step::InstallStep;
use crate::model::config::{Component, InstallConfig, InstallMode};

pub const PACMAN_CORE: &[&str] = &[
    "hyprland",
    "xdg-desktop-portal-hyprland",
    "networkmanager",
    "pipewire",
    "pipewire-pulse",
    "wireplumber",
    "hyprshutdown",
];

pub const PACMAN_CORE_GIT: &[&str] = &[
    "hyprland-git",
    "xdg-desktop-portal-hyprland-git",
    "networkmanager",
    "pipewire",
    "pipewire-pulse",
    "wireplumber",
    "hyprshutdown",
];

pub const PACMAN_DAEMONS: &[&str] = &["swaync", "hypridle", "hyprlock", "polkit-gnome"];

pub const PACMAN_UI: &[&str] = &[
    "waybar",
    "rofi",
    "gtk3",
    "gtk4",
    "qt6-svg",
    "qt6-virtualkeyboard",
    "qt6-multimedia",
    "qt6-multimedia-ffmpeg",
    "hyprwave",
];

pub const PACMAN_UTILS: &[&str] = &[
    "kitty",
    "ghostty",
    "cliphist",
    "nemo",
    "wl-clipboard",
    "hyprshot",
    "brightnessctl",
    "libnotify",
    "ttf-dejavu",
    "noto-fonts",
    "noto-fonts-emoji",
    "awww",
    "papirus-icon-theme",
    "rofi-emoji",
    "ttf-jetbrains-mono-nerd",
    "mise",
    "starship",
    "neovim",
    "sddm",
    "wiremix",
    "bluetui",
    "btop",
    "xcb-util-cursor",
    "evince",
    "parole",
    "eog",
];

pub const PACMAN_BUILD: &[&str] = &[
    "git",
    "base-devel",
    "glib2",
    "uv",
    "sudo-rs",
    "clang",
    "llvm",
    "cmake",
    "pkg-config",
    "curl",
    "wget",
    "unzip",
];

pub const YAY_AUR_PACKAGES: &[&str] = &[
    "nordzy-hyprcursors",
    "zen-browser-bin",
    "weathr-bin",
    "jolt",
    "wlogout",
    "leenfetch",
    "yaru-icon-theme",
    "yaru-gtk-theme",
    "snappy-switcher",
    "hyprmoncfg-bin",
    "quickshell",
];

pub fn build_steps(config: &InstallConfig) -> Vec<InstallStep> {
    let mut steps = Vec::new();

    // 1. Preflight
    steps.push(InstallStep::new(
        "arch_preflight",
        "System Preflight & Environment Check",
        "Verify non-root user privileges, sufficient disk space, and essential base utilities",
    ));

    // 2. Package updates & Pacman packages
    if config.is_component_selected(Component::SystemPackages) {
        steps.push(InstallStep::new(
            "arch_update",
            "Update Pacman Repositories & System",
            "Synchronize package databases and apply current Arch Linux system updates",
        ));

        let mode_desc = match config.mode {
            InstallMode::Stable => "Install official Arch Linux packages (Hyprland, Waybar, Rofi, Audio)",
            InstallMode::Git => "Install bleeding-edge git packages (hyprland-git, wayland-git)",
        };
        steps.push(InstallStep::new(
            "arch_packages",
            "Install Official Pacman Packages",
            mode_desc,
        ));
    }

    // 3. AUR helper & extra packages
    if config.is_component_selected(Component::ExtraRepositories) {
        steps.push(InstallStep::new(
            "arch_aur",
            "Setup AUR Helper & Extra Packages",
            "Ensure yay is available and install Zen Browser, Nordzy cursors, Quickshell, etc.",
        ));
    }

    // 4. SDDM Theme
    if config.is_component_selected(Component::SddmTheme) {
        steps.push(InstallStep::new(
            "arch_sddm",
            "Configure SDDM Astronaut Theme",
            "Clone and configure SDDM astronaut theme, install fonts, and enable sddm.service",
        ));
    }

    // 5. Rust scripts & tools
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "arch_rust_scripts",
            "Build Aurora Rust Scripts & Tools",
            "Compile and install aurora CLI helper scripts from dotfiles, plus termflix and nmrs-tui",
        ));
    }

    // 6. Waytrogen-Aurora
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "arch_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone, build, and deploy waytrogen-aurora wallpaper switcher and schemas",
        ));
    }

    // 7. LazyVim
    if config.is_component_selected(Component::LazyVim) {
        steps.push(InstallStep::new(
            "arch_lazyvim",
            "Deploy LazyVim Neovim Configuration",
            "Backup existing Neovim configs and deploy modern LazyVim starter setup",
        ));
    }

    // 8. Dotfiles
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "arch_dotfiles",
            "Deploy Aurora Hyprland Dotfiles",
            "Backup current configurations and deploy Hyprland, Waybar, SwayNC, Kitty configs",
        ));
    }

    // 9. Shell & PATH
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "arch_shell",
            "Configure Shell & PATH Environment",
            "Ensure ~/.cargo/bin and ~/.local/bin are present in shell PATH profiles",
        ));
    }

    // 10. Sudo-rs
    if config.is_component_selected(Component::SudoRs) && config.allow_sudo_rs {
        steps.push(InstallStep::new(
            "arch_sudo_rs",
            "Switch Sudo to Memory-Safe sudo-rs",
            "Symlink /usr/bin/sudo to /usr/bin/sudo-rs for modern Rust memory safety",
        ));
    }

    // 11. Final theme & polish
    steps.push(InstallStep::new(
        "arch_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}
