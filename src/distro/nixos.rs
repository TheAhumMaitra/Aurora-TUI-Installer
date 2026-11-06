// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroPackageGroup;
use crate::engine::step::InstallStep;
use crate::model::config::{
    BackupStrategy, Component, DisplayManagerChoice, InstallConfig, NeovimPreset,
    VirtualizationChoice,
};

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
    "quickshell",
];

pub const NIXOS_UTILS_PKGS: &[&str] = &[
    "kitty",
    "ghostty",
    "alacritty",
    "foot",
    "starship",
    "neovim",
    "cliphist",
    "wl-clipboard",
    "mise",
    "btop",
    "fish",
    "eog",
    "evince",
];

pub const NIXOS_EXTRAS_PKGS: &[&str] = &[
    "app.zen_browser.zen",
    "bluetui",
    "jolt-tui",
    "leenfetch",
    "nmrs-tui",
    "nordzy-cursors",
    "sddm-astronaut-theme",
    "termflix",
    "waytrogen-aurora",
    "weathr",
];

pub const NIXOS_GAMING_PKGS: &[&str] = &[
    "steam",
    "gamemode",
    "mangohud",
    "gamescope",
];

pub const ALL_NIXOS_PACKAGES: &[&str] = &[
    "alacritty",
    "app.zen_browser.zen",
    "bluetui",
    "btop",
    "cliphist",
    "eog",
    "evince",
    "fish",
    "foot",
    "gamemode",
    "gamescope",
    "ghostty",
    "hypridle",
    "hyprland",
    "hyprlock",
    "jolt-tui",
    "kitty",
    "leenfetch",
    "mangohud",
    "mise",
    "neovim",
    "networkmanager",
    "nmrs-tui",
    "nordzy-cursors",
    "pipewire",
    "quickshell",
    "rofi",
    "sddm-astronaut-theme",
    "starship",
    "steam",
    "swaynotificationcenter",
    "termflix",
    "waybar",
    "waytrogen-aurora",
    "weathr",
    "wireplumber",
    "wl-clipboard",
    "xdg-desktop-portal-hyprland",
];

pub const NIXOS_PACKAGE_GROUPS: &[DistroPackageGroup] = &[
    DistroPackageGroup {
        category: "★ All Packages (Unified)",
        description: "Browse, search & select any specific packages across all categories in one view",
        packages: ALL_NIXOS_PACKAGES,
    },
    DistroPackageGroup {
        category: "Core Compositor (Flake)",
        description: "Hyprland compositor, XDG portal, audio services",
        packages: NIXOS_CORE_PKGS,
    },
    DistroPackageGroup {
        category: "UI & Notification Daemons",
        description: "Waybar status bar, Rofi, SwayNC, Hyprlock, Quickshell",
        packages: NIXOS_UI_PKGS,
    },
    DistroPackageGroup {
        category: "Utilities & Terminal Suite",
        description: "Kitty, Ghostty, Starship, Neovim, Mise, Cliphist, Fish",
        packages: NIXOS_UTILS_PKGS,
    },
    DistroPackageGroup {
        category: "Rice Extras & Theme Assets",
        description: "Zen Browser Flatpak, Nordzy cursors, SDDM astronaut theme, CLI tools",
        packages: NIXOS_EXTRAS_PKGS,
    },
    DistroPackageGroup {
        category: "Gaming & Performance Suite",
        description: "Steam gaming client, GameMode optimization daemon, MangoHud",
        packages: NIXOS_GAMING_PKGS,
    },
];

pub fn package_category_badge(pkg: &str) -> &'static str {
    if NIXOS_CORE_PKGS.contains(&pkg) {
        "Core"
    } else if NIXOS_UI_PKGS.contains(&pkg) {
        "UI"
    } else if NIXOS_UTILS_PKGS.contains(&pkg) {
        "Utils"
    } else if NIXOS_EXTRAS_PKGS.contains(&pkg) {
        "Extras"
    } else if NIXOS_GAMING_PKGS.contains(&pkg) {
        "Gaming"
    } else {
        "Nix"
    }
}

pub fn all_nixos_packages() -> Vec<&'static str> {
    ALL_NIXOS_PACKAGES.to_vec()
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
            "nixos_backup",
            "Archive Existing Dotfiles (Timestamped Backup)",
            "Safely backup existing ~/.config configurations before deploying Aurora",
        ));
    }

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
            format!("Deploy Hyprland (Animation: {}), Waybar ({}), SwayNC, Kitty configs", config.animation_profile.label(), config.bar_layout.label()),
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
    if config.display_manager != DisplayManagerChoice::NoneTty {
        steps.push(InstallStep::new(
            "nixos_sddm",
            "Configure SDDM Astronaut Theme Assets",
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
            "Configure Gaming Suite on NixOS",
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

    // 11. Power & Battery Longevity Suite
    if config.is_component_selected(Component::PowerOptimization) {
        steps.push(InstallStep::new(
            "power_optimization",
            format!("Configure Power Profile ({})", config.power_profile.short_name()),
            format!("Apply {} power management and battery longevity settings", config.power_profile.label()),
        ));
    }

    // 12. Security & Privacy Hardening
    if config.is_component_selected(Component::SecurityHardening) {
        steps.push(InstallStep::new(
            "security_hardening",
            "Apply Security & Privacy Hardening",
            "Configure desktop firewall rules, privacy DNS, and MAC randomization",
        ));
    }

    // 13. Window Effects & Blur Presets
    steps.push(InstallStep::new(
        "window_effects",
        format!("Configure Window Effects ({})", config.window_effects.short_name()),
        format!("Deploy {} decoration and blur shader parameters", config.window_effects.label()),
    ));

    // 14. Cursor & Icon Themes Customizer
    steps.push(InstallStep::new(
        "cursor_icon_themes",
        format!("Deploy Cursor ({}) & Icon ({}) Themes", config.cursor_theme.short_name(), config.icon_theme.short_name()),
        "Configure GTK, desktop icon cache, and Wayland cursor inheritance",
    ));

    // 15. Virtualization Suite
    if config.is_component_selected(Component::Virtualization) && config.virtualization != VirtualizationChoice::None {
        steps.push(InstallStep::new(
            "virtualization_setup",
            format!("Configure Virtualization ({})", config.virtualization.short_name()),
            "Enable hypervisor / container services and grant user group permissions",
        ));
    }

    // 16. Proton-GE & Gaming Enhancements
    if config.proton_ge {
        steps.push(InstallStep::new(
            "proton_ge_setup",
            "Configure Proton-GE & Steam Gaming Enhancements",
            "Setup Steam compatibility tools directory, MangoHud metrics, and GameMode",
        ));
    }

    // 17. Automated Maintenance Timers
    if config.is_component_selected(Component::MaintenanceTimer) && config.auto_maintenance {
        steps.push(InstallStep::new(
            "system_maintenance",
            "Install Automated Maintenance & Health Timers",
            "Deploy systemd user timer for weekly cache cleaning, SSD trim, and dotfile backups",
        ));
    }

    // 18. Theme
    steps.push(InstallStep::new(
        "nixos_theme",
        "Apply Default Aurora Theme",
        format!("Apply theme '{}' globally across desktop components", config.theme_name),
    ));

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nixos_packages_sorted_and_unique() {
        let pkgs = ALL_NIXOS_PACKAGES;
        for i in 1..pkgs.len() {
            assert!(
                pkgs[i - 1] < pkgs[i],
                "ALL_NIXOS_PACKAGES is not sorted or has duplicates: {} >= {}",
                pkgs[i - 1],
                pkgs[i]
            );
        }
    }

    #[test]
    fn test_package_category_badge() {
        assert_eq!(package_category_badge("hyprland"), "Core");
        assert_eq!(package_category_badge("waybar"), "UI");
        assert_eq!(package_category_badge("kitty"), "Utils");
        assert_eq!(package_category_badge("app.zen_browser.zen"), "Extras");
        assert_eq!(package_category_badge("steam"), "Gaming");
    }
}
