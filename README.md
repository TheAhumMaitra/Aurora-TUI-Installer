# Aurora TUI Installer

<div align="center">

```
   ▄████████  ███    █▄     ▄████████  ▄██████▄     ▄████████    ▄████████ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███  ███    ███  ▄███▄▄▄▄██▀ ███    ███  ▄███▄▄▄▄██▀   ███    ███ 
▀███████████  ███    ███ ▀▀███▀▀▀▀▀   ███    ███ ▀▀███▀▀▀▀▀   ▀███████████ 
  ███    ███  ███    ███ ▀███████████ ███    ███ ▀███████████   ███    ███ 
  ███    ███  ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    █▀   ████████▀    ███    ███  ▀██████▀    ███    ███   ███    █▀  
```

**A minimal, elegant, Ratatui-based installer for Aurora.**  
*Supporting Arch Linux, Fedora Linux, and NixOS.*

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with Ratatui](https://img.shields.io/badge/TUI-Ratatui%200.30-cyan.svg)](https://ratatui.rs)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-orange.svg)](https://www.rust-lang.org)

</div>

---

## ✦ Overview

The **Aurora TUI Installer** (`aurora-installer`) provides a responsive, keyboard-driven terminal user interface built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).

It automates the complete configuration and setup of **Aurora** (a minimal, elegant Hyprland preconfigured rice) across the three primary supported Linux distributions:
- **Arch Linux** & derivatives (CachyOS, EndeavourOS, Manjaro, Garuda, ArcoLinux, Archcraft, etc.)
- **Fedora Linux** & derivatives (Fedora Workstation, Nobara, Bazzite, Ultramarine)
- **NixOS** (with Flakes and Nix develop shell integration)

---

## ✦ Extraordinary Customization Options

### 1. Neovim IDE Distribution Selection
Choose between 5 modern development presets (cycled with `[V]` or via `--neovim`):
- **LazyVim Starter** (Aurora official default): Complete preconfigured Neovim environment with Lazy.nvim, LSP, Treesitter, and Which-Key.
- **NvChad**: Blazing fast startup time (<20ms) with rich UI popups, statusline themes, and cheatsheet.
- **AstroNvim**: Feature-rich extensible configuration tailored for modern full-stack development.
- **Kickstart.nvim**: Single-file modular template documented for clean personal customization.
- **None / Keep Existing**: Preserves your existing `~/.config/nvim` untouched.

### 2. Granular Package Selector (`[K]`)
Fine-tune and customize exact packages to be installed:
- Grouped by category: *Core Compositor*, *Daemons & Lockscreen*, *UI & Bar*, *Utilities*, *Build Tools*, *AUR / Extras*, and *Gaming Suite*.
- **Quick Shortcuts**:
  - `[Space]`: Toggle individual package checkbox
  - `[A]`: Select All packages in category
  - `[N]`: Deselect All packages in category
  - `[I]`: Invert selection in category
  - `[Tab] / [←/→]`: Switch between Categories and Packages column
  - `[Enter] / [Esc]`: Save selection and return to Configuration

### 3. Preferred Terminal Emulator Selection
Choose your default terminal (cycled with `[T]` or via `--terminal`):
- **Kitty** (Aurora signature standard)
- **Ghostty** (Ultra-fast modern GPU-accelerated terminal)
- **Alacritty** (Minimalist high-performance OpenGL terminal)
- **Foot** (Lightweight Wayland native terminal)

### 4. Default Login Shell Selection
Choose your shell environment (cycled with `[H]` or via `--shell`):
- **Fish**: Modern shell with smart autosuggestions, completion, and syntax highlighting (default on Fedora).
- **Zsh**: Configured with Starship prompt and high-speed plugins.
- **Bash**: Traditional shell enhanced with Starship prompt.

### 5. Gaming & Performance Suite (`[G]`)
Toggle the dedicated gaming module with `[G]` or `--gaming`:
- Installs **Steam**, **GameMode** (Feral Interactive daemon for CPU/GPU governor priority), **MangoHud** (Vulkan/OpenGL performance overlay), and **Gamescope**.

### 6. Hyprland Animation Speed Profiles (`[O]`)
Cycle animation curves with `[O]`:
- **Snappy** (0.15s instant responsiveness)
- **Smooth** (0.3s fluid cinematic curves)
- **Minimal** (reduced motion for low-latency or battery conservation)

### 7. Live Theme Palette Swatches
The theme selector features live RGB color swatch blocks (`████`) displaying each theme's accent, primary, secondary, and background colors in real time.

---

## ✦ Installation & Build

### Prerequisites
- Rust toolchain (Rust 1.80+ or 2024 edition)
- Cargo

### Building from Source

```bash
cd Aurora-TUI-installer
cargo build --release
```

The optimized binary will be produced at:
```bash
target/release/aurora-installer
```

---

## ✦ Usage

### Interactive TUI Mode (Default)

Launch the installer in your terminal:
```bash
./target/release/aurora-installer
```

### Dry-Run Preview Mode
Preview the entire installation safely without touching your system:
```bash
./target/release/aurora-installer --dry-run
```

### Forcing a Distribution
Force target distro rather than relying on auto-detection:
```bash
./target/release/aurora-installer --distro arch
./target/release/aurora-installer --distro fedora
./target/release/aurora-installer --distro nixos
```

### Non-Interactive / Headless Mode
Automate installation in scripts or CI:
```bash
./target/release/aurora-installer --non-interactive --preset full --neovim nvchad --terminal ghostty --gaming --theme "Aurora Default"
```

### Command-Line Options

```
Usage: aurora-installer [OPTIONS]

Options:
  -d, --distro <DISTRO>      Force target distribution (arch, fedora, nixos)
      --dry-run              Preview actions without applying changes
  -y, --non-interactive      Run without interactive prompts (headless)
  -m, --mode <MODE>          Select release channel (stable vs git)
  -p, --preset <PRESET>      Choose component preset (full, minimal)
      --neovim <NEOVIM>      Choose Neovim preset (lazyvim, nvchad, astronvim, kickstart, none)
      --terminal <TERMINAL>  Choose terminal emulator (kitty, ghostty, alacritty, foot)
      --shell <SHELL>        Choose default shell (fish, zsh, bash)
      --gaming               Enable gaming suite (Steam, GameMode, MangoHud)
  -t, --theme <THEME>        Choose default Aurora theme
      --dir <DIR>            Specify local Aurora dotfiles checkout directory
      --no-sudo-rs           Do not switch sudo to sudo-rs
      --uninstall            Uninstall Aurora configurations and restore backups
      --debug                Enable verbose debug logging
  -h, --help                 Print help
  -V, --version              Print version
```

---

## ✦ TUI Navigation & Keybindings

| Screen | Keybinding | Action |
|---|---|---|
| **All Screens** | `Ctrl + C` | Emergency exit |
| **Welcome** | `D` / `←` / `→` | Cycle target distribution (Arch ↔ Fedora ↔ NixOS) |
| **Welcome** | `Enter` | Proceed to Configuration |
| **Welcome** | `Q` / `Esc` | Quit installer |
| **Configuration** | `K` | **Open Granular Package Selector** |
| **Configuration** | `V` | **Cycle Neovim Preset** (LazyVim ↔ NvChad ↔ AstroNvim ↔ Kickstart ↔ None) |
| **Configuration** | `T` | **Cycle Terminal Emulator** (Kitty ↔ Ghostty ↔ Alacritty ↔ Foot) |
| **Configuration** | `G` | **Toggle Gaming Suite** (Steam, GameMode, MangoHud) |
| **Configuration** | `H` | **Cycle Default Shell** (Fish ↔ Zsh ↔ Bash) |
| **Configuration** | `O` | **Cycle Animation Curves** (Snappy ↔ Smooth ↔ Minimal) |
| **Configuration** | `P` | Cycle preset (Full Rice ↔ Minimal ↔ Custom) |
| **Configuration** | `M` | Cycle release channel (Stable ↔ Bleeding Edge) |
| **Configuration** | `D` | Toggle Dry-Run safe mode |
| **Configuration** | `S` | Toggle memory-safe sudo-rs |
| **Configuration** | `Tab` / `Shift+Tab` | Cycle active section |
| **Configuration** | `↑` / `↓` | Navigate items in Components checklist or Themes list |
| **Configuration** | `Space` | Toggle component, toggle dry-run, or select theme |
| **Configuration** | `Enter` | Review planned pipeline |
| **Configuration** | `Esc` / `B` | Return to Welcome screen |
| **Package Selector** | `Space` | Toggle individual package checkbox |
| **Package Selector** | `A` | **Select All** packages in category |
| **Package Selector** | `N` | **Deselect All** packages in category |
| **Package Selector** | `I` | **Invert** selection in category |
| **Package Selector** | `Tab` / `←` / `→` | Switch between Categories and Packages column |
| **Package Selector** | `Enter` / `Esc` | Save selection and return to Configuration |
| **Summary** | `Enter` / `Y` | **Start installation pipeline** |
| **Summary** | `Esc` / `B` | Go back and adjust settings |
| **Summary** | `Q` | Cancel and quit |
| **Installation** | `S` | Toggle live log auto-scroll |
| **Installation** | `↑` / `↓` / `PgUp` / `PgDn` | Scroll terminal log history |
| **Complete** | `Enter` / `Q` | Exit installer |
| **Complete** | `R` | Reboot system |

---

## ✦ License

This project is licensed under the **GNU General Public License v3.0 or later** ([GPL-3.0-or-later](LICENSE)).

Copyright (C) 2026 Ahum Maitra <theahummaitra@gmail.com>
