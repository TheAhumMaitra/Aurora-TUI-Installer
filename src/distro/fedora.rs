// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{Component, InstallConfig, NeovimPreset};

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
    "ghostty",
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

pub const FEDORA_SOURCE_TOOLS: &[&str] = &[
    "Nordzy-cursors",
    "rofi-emoji",
    "snappy-switcher",
    "hyprwave",
];

pub const FEDORA_GAMING_PACKAGES: &[&str] = &[
    "steam",
    "gamemode",
    "mangohud",
];

pub const FEDORA_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "Core Compositor",
        description: "Hyprland compositor, XDG portals, pipewire audio and session manager",
        packages: DNF_CORE,
    },
    DistroPackageGroup {
        category: "Daemons & Lockscreen",
        description: "Notification center (SwayNC), idle daemon, lockscreen, polkit agent",
        packages: DNF_DAEMONS,
    },
    DistroPackageGroup {
        category: "Bar & UI Components",
        description: "Waybar status bar, Rofi launcher, GTK3/4 themes, Qt6 wayland libraries",
        packages: DNF_UI,
    },
    DistroPackageGroup {
        category: "Utilities & Terminal",
        description: "Kitty, Ghostty, Cliphist, Nemo, Hyprshot, Starship, Mise, fonts",
        packages: DNF_UTILS,
    },
    DistroPackageGroup {
        category: "Build Tools & Compilers",
        description: "Base development tools, GCC, Clang, CMake, Rust, Cargo",
        packages: DNF_BUILD,
    },
    DistroPackageGroup {
        category: "Desktop Addons (Source)",
        description: "Nordzy cursors, rofi-emoji, snappy-switcher, hyprwave",
        packages: FEDORA_SOURCE_TOOLS,
    },
    DistroPackageGroup {
        category: "Gaming & Performance Suite",
        description: "Steam client, GameMode optimization daemon, MangoHud FPS overlay",
        packages: FEDORA_GAMING_PACKAGES,
    },
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

    // 5. Gaming Suite
    if config.is_component_selected(Component::GamingSuite) {
        steps.push(InstallStep::new(
            "fedora_gaming",
            "Install Gaming & Performance Suite",
            "Install Steam, GameMode optimizer daemon, and MangoHud overlay",
        ));
    }

    // 6. SDDM Astronaut Theme
    if config.is_component_selected(Component::SddmTheme) {
        steps.push(InstallStep::new(
            "fedora_sddm",
            "Configure SDDM Astronaut Theme",
            "Clone and configure astronaut theme, install fonts, disable competing DMs, enable SDDM",
        ));
    }

    // 7. Rust scripts
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "fedora_rust_scripts",
            "Build Aurora Rust Scripts & Tools",
            "Compile and install aurora CLI helper scripts from dotfiles, plus termflix and nmrs-tui",
        ));
    }

    // 8. Waytrogen
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "fedora_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone and build waytrogen-aurora wallpaper manager and compile schemas",
        ));
    }

    // 9. Neovim Setup
    if config.is_component_selected(Component::LazyVim) && config.neovim_preset != NeovimPreset::None {
        steps.push(InstallStep::new(
            "fedora_lazyvim",
            format!("Deploy {} Setup", config.neovim_preset.label()),
            format!("Backup existing Neovim configs and deploy {}", config.neovim_preset.description()),
        ));
    }

    // 10. Dotfiles
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "fedora_dotfiles",
            "Deploy Aurora Hyprland Dotfiles",
            format!("Deploy Hyprland (Animation: {}), Waybar, SwayNC, Kitty configs", config.animation_profile.label()),
        ));
    }

    // 11. Shell config
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "fedora_shell",
            format!("Configure Shell ({}) & PATH Environment", config.shell_choice.label()),
            "Configure chosen shell and ensure ~/.cargo/bin and ~/.local/bin are in PATH",
        ));
    }

    // 12. Sudo-rs
    if config.is_component_selected(Component::SudoRs) && config.allow_sudo_rs {
        steps.push(InstallStep::new(
            "fedora_sudo_rs",
            "Switch Sudo to Memory-Safe sudo-rs",
            "Symlink /usr/bin/sudo to /usr/bin/sudo-rs for memory safety",
        ));
    }

    // 13. Final theme & polish
    steps.push(InstallStep::new(
        "fedora_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' and restart Waybar & awww daemons", config.theme_name),
    ));

    steps
}
