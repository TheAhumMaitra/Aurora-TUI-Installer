// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{
    AudioSuiteChoice, BrowserChoice, Component, DisplayManagerChoice, InstallConfig,
    InstallMode, KernelTweakProfile, NeovimPreset,
};

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

pub const ARCH_GAMING_PACKAGES: &[&str] = &[
    "steam",
    "gamemode",
    "mangohud",
    "gamescope",
];

pub const ARCH_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "Core Compositor",
        description: "Hyprland compositor, XDG portals, pipewire audio and session manager",
        packages: PACMAN_CORE,
    },
    DistroPackageGroup {
        category: "Daemons & Lockscreen",
        description: "Notification center (SwayNC), idle daemon, lockscreen, polkit agent",
        packages: PACMAN_DAEMONS,
    },
    DistroPackageGroup {
        category: "Bar & UI Components",
        description: "Waybar status bar, Rofi launcher, GTK3/4 themes, Qt6 wayland libraries",
        packages: PACMAN_UI,
    },
    DistroPackageGroup {
        category: "Utilities & Terminal",
        description: "Kitty, Ghostty, Cliphist, Nemo, Hyprshot, Starship, Mise, Nerd fonts",
        packages: PACMAN_UTILS,
    },
    DistroPackageGroup {
        category: "Build Tools & Compilers",
        description: "Base development tools, GCC, Clang, CMake, Rust, Cargo, sudo-rs",
        packages: PACMAN_BUILD,
    },
    DistroPackageGroup {
        category: "AUR Rice Addons",
        description: "Zen Browser, Nordzy cursors, Quickshell, Snappy Switcher, Weathr",
        packages: YAY_AUR_PACKAGES,
    },
    DistroPackageGroup {
        category: "Gaming & Performance Suite",
        description: "Steam client, GameMode optimization daemon, MangoHud FPS overlay",
        packages: ARCH_GAMING_PACKAGES,
    },
];

pub fn all_arch_packages() -> Vec<&'static str> {
    let mut all = Vec::new();
    for group in ARCH_PACKAGE_GROUPS {
        all.extend_from_slice(group.packages);
    }
    all
}

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
            "Ensure yay/paru is available and install Nordzy cursors, Quickshell, Snappy Switcher, etc.",
        ));
    }

    // 4. Default Web Browser
    if config.browser_choice != BrowserChoice::None {
        steps.push(InstallStep::new(
            "arch_browser",
            format!("Deploy {} Browser", config.browser_choice.short_name()),
            format!("Install and configure {} as the default desktop browser", config.browser_choice.label()),
        ));
    }

    // 5. Gaming Suite (if selected)
    if config.is_component_selected(Component::GamingSuite) {
        steps.push(InstallStep::new(
            "arch_gaming",
            "Install Gaming & Performance Suite",
            "Install Steam, GameMode optimizer daemon, and MangoHud overlay",
        ));
    }

    // 6. Display Manager / Login Screen
    if config.display_manager != DisplayManagerChoice::NoneTty {
        steps.push(InstallStep::new(
            "arch_sddm",
            format!("Configure Display Manager ({})", config.display_manager.short_name()),
            format!("Setup {} login manager and enable systemd display-manager service", config.display_manager.label()),
        ));
    }

    // 7. Audio & Sound Processing Suite
    if config.audio_suite == AudioSuiteChoice::EasyEffectsStudio {
        steps.push(InstallStep::new(
            "arch_audio",
            "Configure EasyEffects Studio DSP Audio",
            "Deploy PipeWire DSP equalizer, bass booster, and voice isolation profiles",
        ));
    }

    // 8. Rust scripts & tools
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "arch_rust_scripts",
            "Build Aurora Rust Scripts & Tools",
            "Compile and install aurora CLI helper scripts from dotfiles, plus termflix and nmrs-tui",
        ));
    }

    // 9. Waytrogen-Aurora
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "arch_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone, build, and deploy waytrogen-aurora wallpaper switcher and schemas",
        ));
    }

    // 10. Neovim Setup
    if config.is_component_selected(Component::LazyVim) && config.neovim_preset != NeovimPreset::None {
        steps.push(InstallStep::new(
            "arch_lazyvim",
            format!("Deploy {} Setup", config.neovim_preset.label()),
            format!("Backup existing Neovim configs and deploy {}", config.neovim_preset.description()),
        ));
    }

    // 11. Dotfiles with Custom Bar Layout & Launcher Styling
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "arch_dotfiles",
            format!("Deploy Aurora Dotfiles (Bar: {}, Rofi: {})", config.bar_layout.short_name(), config.launcher_style.short_name()),
            format!("Deploy Hyprland (Animation: {}), Waybar ({}), SwayNC, and terminal configs", config.animation_profile.label(), config.bar_layout.label()),
        ));
    }

    // 12. Shell & PATH
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "arch_shell",
            format!("Configure Shell ({}) & PATH Environment", config.shell_choice.label()),
            "Ensure ~/.cargo/bin and ~/.local/bin are present in shell PATH profiles",
        ));
    }

    // 13. Kernel & Scheduler Tweaks
    if config.kernel_tweaks == KernelTweakProfile::Performance {
        steps.push(InstallStep::new(
            "arch_tweaks",
            "Apply Kernel & Process Optimizations (Ananicy + ZRAM)",
            "Install and enable ananicy-cpp auto-nice scheduler and zram-generator",
        ));
    }

    // 14. Sudo-rs
    if config.is_component_selected(Component::SudoRs) && config.allow_sudo_rs {
        steps.push(InstallStep::new(
            "arch_sudo_rs",
            "Switch Sudo to Memory-Safe sudo-rs",
            "Symlink /usr/bin/sudo to /usr/bin/sudo-rs for modern Rust memory safety",
        ));
    }

    // 15. Final theme & polish
    steps.push(InstallStep::new(
        "arch_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}
