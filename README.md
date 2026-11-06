# Aurora TUI Installer

A fast, elegant, terminal user interface (TUI) installer for **Aurora**, built with [Ratatui](https://github.com/ratatui/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm) in Rust.

Supports:
- **Arch Linux** & derivatives (EndeavourOS, CachyOS, Manjaro, Garuda, Artix, ArcoLinux, Archcraft)
- **Fedora Linux** & derivatives (Nobara, Bazzite, Ultramarine)
- **NixOS**

---

## Features

- **Multi-Distro Support**: Auto-detects your distribution or allows manual selection across Arch, Fedora, and NixOS.
- **Interactive TUI Wizard**:
  - **1. Welcome & Distro Selection**: ASCII banner, hardware diagnostics (CPU, RAM, free disk space, user privileges, session detection).
  - **2. Configuration & Presets**: Choose between `Full Rice`, `Minimal Core`, or `Custom Selection`. Toggle components (SDDM theme, LazyVim, Waytrogen, Rust scripts, Sudo-rs, Fish shell).
  - **3. Theme Picker**: Pre-configure your preferred default theme (`Aurora Default`, `Vanta Black`, `Osaka Jade`, `Everblush`, `Kanagawa Dragon`, `Your Name`, `Dark Decay`).
  - **4. Review**: Pre-flight checks and complete step pipeline inspection.
  - **5. Live Execution**: Step checklist with real-time status badges, progress gauge, and streaming colored terminal log pane.
  - **6. Completion & Cheatsheet**: Post-installation summary, next steps, and keybindings cheatsheet.
- **Safety First**:
  - Full **Dry-Run mode** (`--dry-run` or toggle in TUI) previews all commands without modifying the system.
  - Automatic timestamped backups to `~/.config/aurora_backup_<timestamp>`.
  - Preserves user custom configs at `~/.config/hypr/User`.
- **Non-Interactive / Unattended Mode**: CLI flags for scripting and automation (`-y` / `--non-interactive`).

---

## Quick Start

### Build & Run

```bash
cd Aurora-TUI-installer
cargo build --release
./target/release/aurora-installer
```

### Command Line Options

```text
Usage: aurora-installer [OPTIONS]

Options:
  -d, --distro <DISTRO>      Force distribution: arch, fedora, nixos
      --dry-run              Preview actions without applying changes
  -y, --non-interactive      Run unattended in headless mode
  -m, --mode <MODE>          Installation mode: stable, git
  -p, --preset <PRESET>      Preset: full, minimal
  -t, --theme <THEME>        Choose default theme (e.g. "Aurora Default", "Vanta Black")
      --dir <DIR>            Custom path to Aurora dotfiles
      --no-sudo-rs           Skip replacing sudo with sudo-rs
      --uninstall            Uninstall Aurora configurations and restore backups
      --debug                Enable verbose debug logging
  -h, --help                 Print help
  -V, --version              Print version
```

### Examples

```bash
# Launch interactive TUI in safe dry-run mode
aurora-installer --dry-run

# Run non-interactive installation on Arch Linux
aurora-installer --distro arch --non-interactive

# Install Fedora edition with minimal preset and Vanta Black theme
aurora-installer --distro fedora --preset minimal --theme "Vanta Black"
```

---

## License

GNU General Public License v3.0 or later (GPL-3.0-or-later)  
Copyright (C) 2026 Ahum Maitra
