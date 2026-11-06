// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::engine::step::InstallStep;
use crate::model::config::{Component, InstallConfig};

pub const FEDORA_COPR_REPOS: &[&str] = &[
    "solopasha/hyprland",
    "sneexy/zen-browser",
    "maveonair/desktop-tools",
    "atim/starship",
    "erikreider/swayosd",
    "solopasha/hyprmoncfg",
];

pub const DNF_CORE: &[&str] = &[
    "hyprland",
    "xdg-desktop-portal-hyprland",
    "NetworkManager",
    "pipewire",
    "pipewire-pulseaudio",
    "wireplumber",
    "hyprshutdown",
];

pub const DNF_DAEMONS: &[&str] = &["swaync", "hypridle", "hyprlock", "polkit-gnome"];

pub const DNF_UI: &[&str] = &[
    "waybar",
    "rofi-wayland",
    "gtk3",
    "gtk4",
    "qt6-qtsvg",
    "qt6-qtdeclarative",
    "qt6-qtmultimedia",
    "qt6-virtualkeyboard",
];

pub const DNF_UTILS: &[&str] = &[
    "kitty",
    "cliphist",
    "nemo",
    "wl-clipboard",
    "hyprshot",
    "brightnessctl",
    "libnotify",
    "dejavu-sans-fonts",
    "google-noto-color-emoji-fonts",
    "papirus-icon-theme",
    "mise",
    "starship",
    "neovim",
    "sddm",
    "fish",
    "eog",
    "evince",
    "parole",
    "btop",
    "swayosd",
    "hyprmoncfg",
];

pub const DNF_BUILD: &[&str] = &[
    "git",
    "gcc",
    "gcc-c++",
    "clang",
    "llvm",
    "cmake",
    "meson",
    "ninja-build",
    "pkgconf-pkg-config",
    "curl",
    "wget",
    "unzip",
    "rust",
    "cargo",
];

pub fn build_steps(config: &InstallConfig) -> Vec<InstallStep> {
    let mut steps = Vec::new();

    // 1. Preflight
    steps.push(InstallStep::new(
        "fedora_preflight",
        "System Preflight & Environment Check",
        "Verify Fedora distribution family, non-root user privileges, and free disk space",
    ));

    // 2. COPR repositories
    if config.is_component_selected(Component::ExtraRepositories) {
        steps.push(InstallStep::new(
            "fedora_copr",
            "Enable Fedora COPR Repositories",
            "Enable COPR repositories for Hyprland (solopasha), Zen Browser, Starship, and fonts",
        ));
    }

    // 3. DNF packages
    if config.is_component_selected(Component::SystemPackages) {
        steps.push(InstallStep::new(
            "fedora_packages",
            "Install Fedora DNF Packages",
            "Install Hyprland, Waybar, SwayNC, Rofi, PipeWire, Ghostty, Kitty, and build toolchain",
        ));
    }

    // 4. Source Builds & Extras (Nordzy, rofi-emoji, snappy-switcher, hyprwave)
    if config.is_component_selected(Component::ExtraRepositories) {
        steps.push(InstallStep::new(
            "fedora_source_tools",
            "Build & Install Desktop Addons",
            "Install Nordzy cursors, hyprwave, rofi-emoji plugin, and snappy-switcher",
        ));
    }

    // 5. SDDM Astronaut Theme
    if config.is_component_selected(Component::SddmTheme) {
        steps.push(InstallStep::new(
            "fedora_sddm",
            "Configure SDDM Astronaut Theme",
            "Clone and configure astronaut theme, install fonts, disable competing DMs, enable SDDM",
        ));
    }

    // 6. Rust scripts
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "fedora_rust_scripts",
            "Build Aurora Rust Scripts & Tools",
            "Compile and install aurora CLI helper scripts from dotfiles, plus termflix and nmrs-tui",
        ));
    }

    // 7. Waytrogen
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "fedora_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone and build waytrogen-aurora wallpaper manager and compile schemas",
        ));
    }

    // 8. LazyVim
    if config.is_component_selected(Component::LazyVim) {
        steps.push(InstallStep::new(
            "fedora_lazyvim",
            "Deploy LazyVim Neovim Configuration",
            "Backup existing Neovim configs and deploy modern LazyVim starter setup",
        ));
    }

    // 9. Dotfiles
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "fedora_dotfiles",
            "Deploy Aurora Hyprland Dotfiles",
            "Backup current configurations and deploy Hyprland, Waybar, SwayNC, Kitty configs",
        ));
    }

    // 10. Shell config
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "fedora_shell",
            "Configure Fish Shell & PATH Environment",
            "Configure default shell to Fish and ensure ~/.cargo/bin and ~/.local/bin are in PATH",
        ));
    }

    // 11. Sudo-rs
    if config.is_component_selected(Component::SudoRs) && config.allow_sudo_rs {
        steps.push(InstallStep::new(
            "fedora_sudo_rs",
            "Switch Sudo to Memory-Safe sudo-rs",
            "Symlink /usr/bin/sudo to /usr/bin/sudo-rs for memory safety",
        ));
    }

    // 12. Final theme & polish
    steps.push(InstallStep::new(
        "fedora_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' and restart Waybar & awww daemons", config.theme_name),
    ));

    steps
}
