// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{
    AudioSuiteChoice, BackupStrategy, BrowserChoice, Component, DisplayManagerChoice,
    GpuProfile, InstallConfig, InstallMode, KernelTweakProfile, NeovimPreset,
    VirtualizationChoice,
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

pub const ALL_ARCH_PACKAGES: &[&str] = &[
    "alacritty",
    "awww",
    "base-devel",
    "bluetui",
    "brightnessctl",
    "btop",
    "clang",
    "cliphist",
    "cmake",
    "curl",
    "egl-wayland",
    "eog",
    "evince",
    "foot",
    "gamemode",
    "gamescope",
    "ghostty",
    "git",
    "glib2",
    "gtk3",
    "gtk4",
    "hypridle",
    "hyprland",
    "hyprlock",
    "hyprmoncfg-bin",
    "hyprpaper",
    "hyprshot",
    "hyprshutdown",
    "hyprwave",
    "jolt",
    "kitty",
    "leenfetch",
    "libnotify",
    "llvm",
    "mangohud",
    "mise",
    "mpvpaper",
    "nemo",
    "neovim",
    "networkmanager",
    "nordzy-hyprcursors",
    "noto-fonts",
    "noto-fonts-emoji",
    "nvidia-dkms",
    "nvidia-utils",
    "papirus-icon-theme",
    "parole",
    "pipewire",
    "pipewire-pulse",
    "pkg-config",
    "polkit-gnome",
    "qt6-multimedia",
    "qt6-multimedia-ffmpeg",
    "qt6-svg",
    "qt6-virtualkeyboard",
    "quickshell",
    "rofi",
    "rofi-emoji",
    "sddm",
    "snappy-switcher",
    "starship",
    "steam",
    "sudo-rs",
    "swaylock",
    "swaync",
    "swww",
    "ttf-cascadia-code-nerd",
    "ttf-dejavu",
    "ttf-firacode-nerd",
    "ttf-hack-nerd",
    "ttf-jetbrains-mono-nerd",
    "unzip",
    "uv",
    "waybar",
    "weathr-bin",
    "wget",
    "wiremix",
    "wireplumber",
    "wl-clipboard",
    "wlogout",
    "xcb-util-cursor",
    "xdg-desktop-portal-hyprland",
    "yaru-gtk-theme",
    "yaru-icon-theme",
    "zen-browser-bin",
];

pub const ARCH_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "★ All Packages (Unified)",
        description: "Browse, search & select any specific packages across all categories in one view",
        packages: ALL_ARCH_PACKAGES,
    },
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

pub fn package_category_badge(pkg: &str) -> &'static str {
    if pkg.starts_with("ttf-") || pkg.starts_with("noto-") {
        "Font"
    } else if pkg.starts_with("nvidia-") || pkg == "egl-wayland" {
        "GPU"
    } else if pkg == "swww" || pkg == "hyprpaper" || pkg == "mpvpaper" || pkg == "awww" {
        "Wallpaper"
    } else if PACMAN_CORE.contains(&pkg) {
        "Core"
    } else if PACMAN_DAEMONS.contains(&pkg) {
        "Daemon"
    } else if PACMAN_UI.contains(&pkg) {
        "UI"
    } else if PACMAN_UTILS.contains(&pkg) {
        "Utils"
    } else if PACMAN_BUILD.contains(&pkg) {
        "Build"
    } else if YAY_AUR_PACKAGES.contains(&pkg) {
        "AUR"
    } else if ARCH_GAMING_PACKAGES.contains(&pkg) {
        "Gaming"
    } else {
        "Extra"
    }
}

pub fn all_arch_packages() -> Vec<&'static str> {
    ALL_ARCH_PACKAGES.to_vec()
}

pub fn build_steps(config: &InstallConfig) -> Vec<InstallStep> {
    let mut steps = Vec::new();

    // 0a. Btrfs Snapshot (Instant Rollback Hook)
    if config.btrfs_snapshot {
        steps.push(InstallStep::new(
            "btrfs_snapshot",
            "Create Pre-Installation Btrfs Snapshot",
            "Create instant rollback snapshot before applying system changes",
        ));
    }

    // 0b. Backup existing configs if requested
    if config.backup_strategy == BackupStrategy::Timestamped {
        steps.push(InstallStep::new(
            "arch_backup",
            "Archive Existing Dotfiles (Timestamped Backup)",
            "Safely backup existing ~/.config configurations before deploying Aurora",
        ));
    }

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
        steps.push(InstallStep::new(
            "arch_network_manager",
            "Configure NetworkManager Service",
            "Disable competing network daemons (iwd, systemd-networkd, connman) and ensure NetworkManager is primary",
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

    // 11. GPU Acceleration (NVIDIA DKMS & Wayland Envs)
    if config.gpu_profile == GpuProfile::NvidiaProprietary {
        steps.push(InstallStep::new(
            "arch_gpu",
            "Configure NVIDIA Drivers & Wayland Acceleration Envs",
            "Install nvidia-dkms, egl-wayland, and inject hardware acceleration variables into Hyprland",
        ));
    }

    // 12. Wallpaper Engine
    steps.push(InstallStep::new(
        "arch_wallpaper",
        format!("Deploy Wallpaper Engine ({})", config.wallpaper_engine.short_name()),
        format!("Setup {} wallpaper daemon for desktop background management", config.wallpaper_engine.label()),
    ));

    // 13. Dotfiles with Custom Bar Layout & Launcher Styling
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

    // 15. Power & Battery Longevity Suite
    if config.is_component_selected(Component::PowerOptimization) {
        steps.push(InstallStep::new(
            "power_optimization",
            format!("Configure Power Profile ({})", config.power_profile.short_name()),
            format!("Apply {} power management and battery longevity settings", config.power_profile.label()),
        ));
    }

    // 16. Security & Privacy Hardening
    if config.is_component_selected(Component::SecurityHardening) {
        steps.push(InstallStep::new(
            "security_hardening",
            "Apply Security & Privacy Hardening",
            "Configure desktop firewall rules, privacy DNS, and MAC randomization",
        ));
    }

    // 17. Window Effects & Blur Presets
    steps.push(InstallStep::new(
        "window_effects",
        format!("Configure Window Effects ({})", config.window_effects.short_name()),
        format!("Deploy {} decoration and blur shader parameters", config.window_effects.label()),
    ));

    // 18. Cursor & Icon Themes Customizer
    steps.push(InstallStep::new(
        "cursor_icon_themes",
        format!("Deploy Cursor ({}) & Icon ({}) Themes", config.cursor_theme.short_name(), config.icon_theme.short_name()),
        "Configure GTK, desktop icon cache, and Wayland cursor inheritance",
    ));

    // 19. Virtualization Suite
    if config.is_component_selected(Component::Virtualization) && config.virtualization != VirtualizationChoice::None {
        steps.push(InstallStep::new(
            "virtualization_setup",
            format!("Configure Virtualization ({})", config.virtualization.short_name()),
            "Enable hypervisor / container services and grant user group permissions",
        ));
    }

    // 20. Proton-GE & Gaming Enhancements
    if config.proton_ge {
        steps.push(InstallStep::new(
            "proton_ge_setup",
            "Configure Proton-GE & Steam Gaming Enhancements",
            "Setup Steam compatibility tools directory, MangoHud metrics, and GameMode",
        ));
    }

    // 21. Automated Maintenance Timers
    if config.is_component_selected(Component::MaintenanceTimer) && config.auto_maintenance {
        steps.push(InstallStep::new(
            "system_maintenance",
            "Install Automated Maintenance & Health Timers",
            "Deploy systemd user timer for weekly cache cleaning, SSD trim, and dotfile backups",
        ));
    }

    // 22. Final theme & polish
    steps.push(InstallStep::new(
        "arch_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_arch_packages_sorted_and_unique() {
        let pkgs = ALL_ARCH_PACKAGES;
        for i in 1..pkgs.len() {
            assert!(
                pkgs[i - 1] < pkgs[i],
                "ALL_ARCH_PACKAGES is not sorted or has duplicates: {} >= {}",
                pkgs[i - 1],
                pkgs[i]
            );
        }
    }

    #[test]
    fn test_package_category_badge() {
        assert_eq!(package_category_badge("hyprland"), "Core");
        assert_eq!(package_category_badge("hypridle"), "Daemon");
        assert_eq!(package_category_badge("waybar"), "UI");
        assert_eq!(package_category_badge("ttf-jetbrains-mono-nerd"), "Font");
        assert_eq!(package_category_badge("zen-browser-bin"), "AUR");
        assert_eq!(package_category_badge("steam"), "Gaming");
        assert_eq!(package_category_badge("git"), "Build");
        assert_eq!(package_category_badge("cliphist"), "Utils");
        assert_eq!(package_category_badge("nvidia-dkms"), "GPU");
        assert_eq!(package_category_badge("swww"), "Wallpaper");
    }
}
