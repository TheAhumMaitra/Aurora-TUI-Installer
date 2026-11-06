// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DistroArg {
    #[value(name = "arch", alias = "archlinux")]
    Arch,
    #[value(name = "fedora")]
    Fedora,
    #[value(name = "nixos", alias = "nix")]
    NixOS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ModeArg {
    #[value(name = "stable")]
    Stable,
    #[value(name = "git")]
    Git,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PresetArg {
    #[value(name = "full")]
    Full,
    #[value(name = "minimal")]
    Minimal,
}

/// Aurora Universal TUI Installer for Arch Linux, Fedora, and NixOS
#[derive(Parser, Debug, Clone)]
#[command(
    name = "aurora-installer",
    author = "Ahum Maitra <theahummaitra@gmail.com>",
    version = "1.0.0",
    about = "A minimal, elegant, Ratatui-based installer for Aurora (Arch, Fedora, NixOS)",
    long_about = "Aurora Installer provides a rich, responsive terminal user interface (TUI) \
                  built on Ratatui to configure and install the Aurora Hyprland rice across \
                  Arch Linux, Fedora, and NixOS."
)]
pub struct CliArgs {
    /// Force a specific Linux distribution (arch, fedora, nixos) instead of auto-detecting
    #[arg(short, long, value_enum)]
    pub distro: Option<DistroArg>,

    /// Preview actions and commands without applying any changes
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Run installation in non-interactive / unattended mode (skips TUI)
    #[arg(short = 'y', long = "non-interactive", alias = "yes", default_value_t = false)]
    pub non_interactive: bool,

    /// Choose installation mode (stable vs bleeding-edge git packages)
    #[arg(short, long, value_enum)]
    pub mode: Option<ModeArg>,

    /// Choose component preset (full rice or minimal desktop)
    #[arg(short, long, value_enum)]
    pub preset: Option<PresetArg>,

    /// Choose the default Aurora theme to apply
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Directory containing the Aurora repository / dotfiles (defaults to local detection)
    #[arg(long)]
    pub dir: Option<PathBuf>,

    /// Skip replacing sudo with sudo-rs
    #[arg(long, default_value_t = false)]
    pub no_sudo_rs: bool,

    /// Uninstall Aurora configurations and restore backups
    #[arg(long, default_value_t = false)]
    pub uninstall: bool,

    /// Enable verbose debug logging output
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}
