// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum NeovimArg {
    #[value(name = "lazyvim")]
    LazyVim,
    #[value(name = "nvchad")]
    NvChad,
    #[value(name = "astronvim")]
    AstroNvim,
    #[value(name = "kickstart")]
    Kickstart,
    #[value(name = "none")]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum TerminalArg {
    #[value(name = "kitty")]
    Kitty,
    #[value(name = "ghostty")]
    Ghostty,
    #[value(name = "alacritty")]
    Alacritty,
    #[value(name = "foot")]
    Foot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ShellArg {
    #[value(name = "fish")]
    Fish,
    #[value(name = "zsh")]
    Zsh,
    #[value(name = "bash")]
    Bash,
}

/// Aurora TUI Installer tailored for Arch Linux and Arch-based distributions
#[derive(Parser, Debug, Clone)]
#[command(
    name = "aurora-installer",
    author = "Ahum Maitra <theahummaitra@gmail.com>",
    version = "1.0.0",
    about = "A minimal, elegant, Ratatui-based installer for Aurora (Arch Linux & Arch-based OS)",
    long_about = "Aurora Installer provides a rich, responsive terminal user interface (TUI) \
                  built on Ratatui to configure and install the Aurora Hyprland rice. \
                  Auto-detects Arch Linux and Arch-based distributions (CachyOS, EndeavourOS, \
                  Manjaro, Garuda, Artix, ArcoLinux, Archcraft, SteamOS) and provides hardware \
                  and distribution compatibility checks."
)]
pub struct CliArgs {
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

    /// Choose the default Neovim distribution preset (lazyvim, nvchad, astronvim, kickstart, none)
    #[arg(long, value_enum)]
    pub neovim: Option<NeovimArg>,

    /// Choose preferred terminal emulator (kitty, ghostty, alacritty, foot)
    #[arg(long, value_enum)]
    pub terminal: Option<TerminalArg>,

    /// Choose preferred default login shell (fish, zsh, bash)
    #[arg(long, value_enum)]
    pub shell: Option<ShellArg>,

    /// Enable gaming and performance suite (Steam, GameMode, MangoHud)
    #[arg(long, default_value_t = false)]
    pub gaming: bool,

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
