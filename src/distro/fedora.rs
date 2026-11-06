// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{
    BackupStrategy, BrowserChoice, Component, DisplayManagerChoice, InstallConfig, InstallMode,
    NeovimPreset, VirtualizationChoice,
};

pub const FEDORA_COPR_REPOS: &[&str] = &[
    "lionheartp/Hyprland",
    "sneexy/zen-browser",
    "maveonair/jetbrains-mono-nerd-fonts",
    "maveonair/desktop-tools",
    "atim/starship",
    "erikreider/swayosd",
    "paolino/hyprmoncfg",
    "solopasha/hyprland",
];

pub const DNF_CORE: &[&str] = &[
    "hyprland",
    "xdg-desktop-portal-hyprland",
    "xdg-desktop-portal-gtk",
    "aquamarine",
    "hyprlang",
    "hyprutils",
    "hyprgraphics",
    "hyprcursor",
    "hyprland-protocols",
    "hypridle",
    "hyprlock",
    "hyprshutdown",
    "hyprpicker",
    "hyprshot",
    "hyprsunset",
    "awww",
    "swayosd",
    "hyprmoncfg",
    "cliphist",
    "NetworkManager",
    "NetworkManager-wifi",
    "pipewire",
    "pipewire-pulseaudio",
    "wireplumber",
];

pub const DNF_CORE_GIT: &[&str] = &[
    "hyprland-git",
    "xdg-desktop-portal-hyprland",
    "xdg-desktop-portal-gtk",
    "aquamarine",
    "hyprlang",
    "hyprutils",
    "hyprgraphics",
    "hyprcursor",
    "hyprland-protocols",
    "hypridle",
    "hyprlock",
    "hyprshutdown",
    "hyprpicker",
    "hyprshot",
    "hyprsunset",
    "awww",
    "swayosd",
    "hyprmoncfg",
    "cliphist",
    "NetworkManager",
    "NetworkManager-wifi",
    "pipewire",
    "pipewire-pulseaudio",
    "wireplumber",
];

pub const DNF_DAEMONS: &[&str] = &[
    "SwayNotificationCenter",
    "polkit-gnome",
    "hyprpolkitagent",
];

pub const DNF_UI: &[&str] = &[
    "waybar",
    "quickshell",
    "rofi",
    "wlogout",
    "nwg-dock-hyprland",
    "gtk3",
    "gtk4",
    "qt6-qtsvg",
    "qt6-qtvirtualkeyboard",
    "qt6-qtmultimedia",
    "qt6ct",
];

pub const DNF_UTILS: &[&str] = &[
    "kitty",
    "ghostty",
    "alacritty",
    "foot",
    "nemo",
    "wl-clipboard",
    "grim",
    "slurp",
    "brightnessctl",
    "libnotify",
    "bluez",
    "bluez-tools",
    "util-linux-user",
    "curl",
    "xsel",
    "dejavu-sans-fonts",
    "google-noto-sans-fonts",
    "google-noto-color-emoji-fonts",
    "papirus-icon-theme",
    "yaru-theme",
    "yaru-icon-theme",
    "jetbrains-mono-nerd-fonts",
    "starship",
    "neovim",
    "sddm",
    "btop",
    "fish",
    "zen-browser",
    "uv",
    "sudo-rs",
    "xorg-x11-server-Xwayland",
    "pipewire-utils",
    "pipewire-alsa",
    "pipewire-jack-audio-connection-kit",
    "evince",
    "eog",
    "parole",
    "mise",
];

pub const DNF_BUILD: &[&str] = &[
    "git",
    "rustup",
    "rust",
    "cargo",
    "rust-analyzer",
    "clippy",
    "rustfmt",
    "gcc",
    "gcc-c++",
    "glibc-devel",
    "kernel-headers",
    "binutils",
    "make",
    "cmake",
    "ninja-build",
    "meson",
    "pkgconf-pkg-config",
    "autoconf",
    "automake",
    "libtool",
    "m4",
    "clang",
    "llvm",
    "lld",
    "compiler-rt",
    "libomp-devel",
    "python3",
    "python3-devel",
    "lua",
    "luajit",
    "gettext",
    "gettext-devel",
    "ncurses-devel",
    "libffi-devel",
    "openssl-devel",
    "cairo-devel",
    "pango-devel",
    "gdk-pixbuf2-devel",
    "gtk4-devel",
    "gtk4-layer-shell-devel",
    "wayland-devel",
    "wayland-protocols-devel",
    "json-c-devel",
    "libxkbcommon-devel",
    "librsvg2-devel",
    "dbus-devel",
    "systemd-devel",
    "glib2-devel",
    "pipewire-devel",
    "rofi-devel",
    "pulseaudio-libs-devel",
    "glib-networking",
];

pub const FEDORA_SOURCE_TOOLS: &[&str] = &[
    "hyprwave",
    "Nordzy-cursors",
    "rofi-emoji",
    "snappy-switcher",
    "waytrogen-aurora",
];

pub const FEDORA_GAMING_PACKAGES: &[&str] = &[
    "steam",
    "gamemode",
    "mangohud",
    "gamescope",
];

pub const ALL_FEDORA_PACKAGES: &[&str] = &[
    "NetworkManager",
    "NetworkManager-wifi",
    "Nordzy-cursors",
    "SwayNotificationCenter",
    "alacritty",
    "aquamarine",
    "autoconf",
    "automake",
    "awww",
    "binutils",
    "bluez",
    "bluez-tools",
    "brightnessctl",
    "btop",
    "cairo-devel",
    "cargo",
    "clang",
    "cliphist",
    "clippy",
    "cmake",
    "compiler-rt",
    "curl",
    "dbus-devel",
    "dejavu-sans-fonts",
    "eog",
    "evince",
    "fish",
    "foot",
    "gamemode",
    "gamescope",
    "gcc",
    "gcc-c++",
    "gdk-pixbuf2-devel",
    "gettext",
    "gettext-devel",
    "ghostty",
    "git",
    "glib-networking",
    "glib2-devel",
    "glibc-devel",
    "google-noto-color-emoji-fonts",
    "google-noto-sans-fonts",
    "grim",
    "gtk3",
    "gtk4",
    "gtk4-devel",
    "gtk4-layer-shell-devel",
    "hyprcursor",
    "hyprgraphics",
    "hypridle",
    "hyprland",
    "hyprland-protocols",
    "hyprlang",
    "hyprlock",
    "hyprmoncfg",
    "hyprpicker",
    "hyprpolkitagent",
    "hyprshot",
    "hyprshutdown",
    "hyprsunset",
    "hyprutils",
    "hyprwave",
    "jetbrains-mono-nerd-fonts",
    "json-c-devel",
    "kernel-headers",
    "kitty",
    "libffi-devel",
    "libnotify",
    "libomp-devel",
    "librsvg2-devel",
    "libtool",
    "libxkbcommon-devel",
    "lld",
    "llvm",
    "lua",
    "luajit",
    "m4",
    "make",
    "mangohud",
    "meson",
    "mise",
    "ncurses-devel",
    "nemo",
    "neovim",
    "ninja-build",
    "nwg-dock-hyprland",
    "openssl-devel",
    "pango-devel",
    "papirus-icon-theme",
    "parole",
    "pipewire",
    "pipewire-alsa",
    "pipewire-devel",
    "pipewire-jack-audio-connection-kit",
    "pipewire-pulseaudio",
    "pipewire-utils",
    "pkgconf-pkg-config",
    "polkit-gnome",
    "pulseaudio-libs-devel",
    "python3",
    "python3-devel",
    "qt6-qtmultimedia",
    "qt6-qtsvg",
    "qt6-qtvirtualkeyboard",
    "qt6ct",
    "quickshell",
    "rofi",
    "rofi-devel",
    "rofi-emoji",
    "rust",
    "rust-analyzer",
    "rustfmt",
    "rustup",
    "sddm",
    "slurp",
    "snappy-switcher",
    "starship",
    "steam",
    "sudo-rs",
    "swayosd",
    "systemd-devel",
    "util-linux-user",
    "uv",
    "waybar",
    "wayland-devel",
    "wayland-protocols-devel",
    "waytrogen-aurora",
    "wireplumber",
    "wl-clipboard",
    "wlogout",
    "xdg-desktop-portal-gtk",
    "xdg-desktop-portal-hyprland",
    "xorg-x11-server-Xwayland",
    "xsel",
    "yaru-icon-theme",
    "yaru-theme",
    "zen-browser",
];

pub const FEDORA_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "★ All Packages (Unified)",
        description: "Browse, search & select any specific packages across all categories in one view",
        packages: ALL_FEDORA_PACKAGES,
    },
    DistroPackageGroup {
        category: "Core Compositor (Hyprland & COPR)",
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
        category: "Utilities & Terminal Suite",
        description: "Kitty, Ghostty, Cliphist, Nemo, Hyprshot, Starship, Mise, fonts",
        packages: DNF_UTILS,
    },
    DistroPackageGroup {
        category: "Build Tools & Toolchain",
        description: "Base development tools, GCC, Clang, CMake, Rust, Cargo",
        packages: DNF_BUILD,
    },
    DistroPackageGroup {
        category: "Desktop Addons (Source)",
        description: "Nordzy cursors, rofi-emoji, snappy-switcher, hyprwave, waytrogen-aurora",
        packages: FEDORA_SOURCE_TOOLS,
    },
    DistroPackageGroup {
        category: "Gaming & Performance Suite",
        description: "Steam client, GameMode optimization daemon, MangoHud FPS overlay",
        packages: FEDORA_GAMING_PACKAGES,
    },
];

pub fn package_category_badge(pkg: &str) -> &'static str {
    if pkg.ends_with("-fonts") || pkg.contains("font") {
        "Font"
    } else if DNF_CORE.contains(&pkg) || pkg == "hyprland-git" {
        "Core"
    } else if DNF_DAEMONS.contains(&pkg) {
        "Daemon"
    } else if DNF_UI.contains(&pkg) {
        "UI"
    } else if DNF_UTILS.contains(&pkg) {
        "Utils"
    } else if DNF_BUILD.contains(&pkg) {
        "Build"
    } else if FEDORA_SOURCE_TOOLS.contains(&pkg) {
        "Source"
    } else if FEDORA_GAMING_PACKAGES.contains(&pkg) {
        "Gaming"
    } else {
        "Extra"
    }
}

pub fn all_fedora_packages() -> Vec<&'static str> {
    ALL_FEDORA_PACKAGES.to_vec()
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
            "fedora_backup",
            "Archive Existing Dotfiles (Timestamped Backup)",
            "Safely backup existing ~/.config configurations before deploying Aurora",
        ));
    }

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
            "Enable COPR repositories for Hyprland (lionheartp), Zen Browser, Starship, and fonts",
        ));
    }

    // 3. DNF packages
    if config.is_component_selected(Component::SystemPackages) {
        let mode_desc = match config.mode {
            InstallMode::Stable => "Install release packages (Hyprland, Waybar, SwayNC, Rofi, PipeWire, build tools)",
            InstallMode::Git => "Install bleeding-edge git packages (hyprland-git, waybar-git)",
        };
        steps.push(InstallStep::new(
            "fedora_packages",
            "Install Fedora DNF Packages",
            mode_desc,
        ));
    }

    // 4. NetworkManager
    steps.push(InstallStep::new(
        "fedora_network_manager",
        "Configure NetworkManager Service",
        "Disable competing network daemons and ensure NetworkManager is primary",
    ));

    // 5. SDDM Astronaut Theme
    if config.display_manager != DisplayManagerChoice::NoneTty {
        steps.push(InstallStep::new(
            "fedora_sddm",
            "Configure SDDM Astronaut Theme & Greeter",
            "Clone astronaut theme, install fonts, disable competing display managers (GDM), and enable SDDM",
        ));
    }

    // 6. Source Builds & Extras (Nordzy, rofi-emoji, snappy-switcher, hyprwave)
    if config.is_component_selected(Component::ExtraRepositories) {
        steps.push(InstallStep::new(
            "fedora_source_tools",
            "Build & Install Desktop Addons",
            "Build and install Nordzy cursors, hyprwave, rofi-emoji plugin, and snappy-switcher",
        ));
    }

    // 7. Rust scripts & CLI tools
    if config.is_component_selected(Component::RustScripts) {
        steps.push(InstallStep::new(
            "fedora_rust_scripts",
            "Build Aurora Rust Scripts & Tools",
            "Compile aurora CLI helper scripts from dotfiles, install mise, termflix, and nmrs-tui",
        ));
    }

    // 8. Waytrogen
    if config.is_component_selected(Component::Waytrogen) {
        steps.push(InstallStep::new(
            "fedora_waytrogen",
            "Install Waytrogen-Aurora Wallpaper Switcher",
            "Clone and build waytrogen-aurora wallpaper manager and compile GLib schemas",
        ));
    }

    // 9. Gaming Suite (if selected)
    if config.is_component_selected(Component::GamingSuite) {
        steps.push(InstallStep::new(
            "fedora_gaming",
            "Install Gaming & Performance Suite",
            "Install Steam, GameMode optimizer daemon, and MangoHud overlay via DNF",
        ));
    }

    // 10. Default Web Browser
    if config.browser_choice != BrowserChoice::None {
        steps.push(InstallStep::new(
            "fedora_browser",
            format!("Deploy {} Browser", config.browser_choice.short_name()),
            format!("Ensure {} is installed and configured as desktop browser", config.browser_choice.label()),
        ));
    }

    // 11. Neovim Setup
    if config.is_component_selected(Component::LazyVim) && config.neovim_preset != NeovimPreset::None {
        steps.push(InstallStep::new(
            "fedora_lazyvim",
            format!("Deploy {} Setup", config.neovim_preset.label()),
            format!("Backup existing Neovim configs and deploy {}", config.neovim_preset.description()),
        ));
    }

    // 12. Dotfiles
    if config.is_component_selected(Component::Dotfiles) {
        steps.push(InstallStep::new(
            "fedora_dotfiles",
            "Deploy Aurora Hyprland Dotfiles",
            format!("Deploy Hyprland (Animation: {}), Waybar ({}), SwayNC, and terminal configs", config.animation_profile.label(), config.bar_layout.label()),
        ));
    }

    // 13. Shell config & Fish default
    if config.is_component_selected(Component::ShellConfig) {
        steps.push(InstallStep::new(
            "fedora_shell",
            format!("Configure Shell ({}) & PATH Environment", config.shell_choice.label()),
            "Ensure ~/.cargo/bin and ~/.local/bin in PATH and configure Fish default shell",
        ));
    }

    // 14. Sudo-rs
    if config.is_component_selected(Component::SudoRs) && config.allow_sudo_rs {
        steps.push(InstallStep::new(
            "fedora_sudo_rs",
            "Switch Sudo to Memory-Safe sudo-rs",
            "Symlink /usr/bin/sudo to /usr/bin/sudo-rs with safe backup",
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
        "fedora_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally, launch awww-daemon, and restart Waybar", config.theme_name),
    ));

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_fedora_packages_sorted_and_unique() {
        let pkgs = ALL_FEDORA_PACKAGES;
        for i in 1..pkgs.len() {
            assert!(
                pkgs[i - 1] < pkgs[i],
                "ALL_FEDORA_PACKAGES is not sorted or has duplicates: {} >= {}",
                pkgs[i - 1],
                pkgs[i]
            );
        }
    }

    #[test]
    fn test_package_category_badge() {
        assert_eq!(package_category_badge("hyprland"), "Core");
        assert_eq!(package_category_badge("SwayNotificationCenter"), "Daemon");
        assert_eq!(package_category_badge("waybar"), "UI");
        assert_eq!(package_category_badge("jetbrains-mono-nerd-fonts"), "Font");
        assert_eq!(package_category_badge("steam"), "Gaming");
        assert_eq!(package_category_badge("git"), "Build");
        assert_eq!(package_category_badge("hyprwave"), "Source");
    }
}
