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
*Tailored for Arch Linux & Arch-based distributions.*

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with Ratatui](https://img.shields.io/badge/TUI-Ratatui%200.30-cyan.svg)](https://ratatui.rs)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-orange.svg)](https://www.rust-lang.org)

</div>

---

## ✦ Overview

The **Aurora TUI Installer** (`aurora-installer`) provides a responsive, keyboard-driven terminal user interface built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).

Aurora is tailored for **Arch Linux and all Arch-based distributions** ("ARCHOS based distros"). Manual distribution selection is removed: the installer automatically detects the running operating system and verifies hardware and distribution compatibility on launch:
- **Pure Arch Linux** (`arch`, `archlinux`)
- **CachyOS** (Kernel-optimized performance Arch derivative)
- **EndeavourOS** (Popular desktop Arch derivative)
- **Manjaro Linux** & **Manjaro ARM**
- **Garuda Linux** (Gaming and performance oriented)
- **Artix Linux**
- **ArcoLinux**
- **Archcraft**
- **BlackArch**, **SteamOS**, and any system with `pacman` or `arch` in `ID_LIKE`.

---

## ✦ Automatic Distribution Compatibility & Health Check

Upon launching, the installer automatically validates:
1. **Arch Ecosystem Compatibility**: Identifies your exact Arch flavor, verifies `pacman` availability, and detects whether an AUR helper (`yay` or `paru`) is installed or needs automatic bootstrapping.
2. **User Privileges**: Checks standard non-root execution with `sudo` capabilities.
3. **Storage Requirements**: Verifies available free space in `$HOME` (>= 5 GB recommended).
4. **Memory & CPU**: Validates logical cores and memory capacity.
5. **Session Type**: Detects active Wayland / Hyprland sessions.
6. **Dotfiles Safety**: Protects custom configurations (`~/.config/hypr/User` is always preserved).

If run on an unsupported non-Arch distribution (e.g. Fedora, Ubuntu, Debian), a clear diagnostic card is displayed explaining the requirement for Arch package management (`pacman`/AUR), with live installation restricted to avoid system conflicts while enabling `--dry-run` preview.

---

## ✦ Extraordinary Customization Options

### 1. Custom Preset & Global Package Select / Deselect
Fine-tune and customize exact packages to be installed, with instant bulk controls:
- **Global Bulk Controls**:
  - `[U]` or `[Shift+A]` or `[Ctrl+A]`: **SELECT ALL PACKAGES GLOBALLY** (across all categories)
  - `[X]` or `[Shift+N]` or `[Ctrl+D]`: **DESELECT ALL PACKAGES GLOBALLY** (minimal foundation)
  - In Configuration Preset card: Press `[A]` to Enable All, `[X]` to Disable All.
- **Category-Level Shortcuts (`[K]` Package Customizer)**:
  - `[Space]`: Toggle individual package checkbox
  - `[A]`: Select All packages in active category
  - `[N]`: Deselect All packages in active category
  - `[I]`: Invert selection in active category
  - `[Tab] / [←/→]`: Switch between Categories and Packages column
  - `[Enter] / [Esc]`: Save selection and return to Configuration

### 2. Neovim IDE Distribution Selection
Choose between 5 modern development presets (cycled with `[V]` or via `--neovim`):
- **LazyVim Starter** (Aurora official default): Complete preconfigured Neovim environment with Lazy.nvim, LSP, Treesitter, and Which-Key.
- **NvChad**: Blazing fast startup time (<20ms) with rich UI popups, statusline themes, and cheatsheet.
- **AstroNvim**: Feature-rich extensible configuration tailored for modern full-stack development.
- **Kickstart.nvim**: Single-file modular template documented for clean personal customization.
- **None / Keep Existing**: Preserves your existing `~/.config/nvim` untouched.

### 3. Default Web Browser Selection
Choose your preferred browser (cycled with `[W]` or via `--browser`):
- **Zen Browser** (Aurora Signature - vertical split tabs, Arc-like workflow, privacy-first)
- **Brave** (Native adblocking & privacy shield)
- **Firefox** (Official Wayland open-source standard)
- **Chromium** (Pure engine & high compatibility)
- **None** (Preserve current browser untouched)

### 4. Display Manager / Login Screen Selection
Choose how you log into Hyprland (cycled with `[L]` or via `--dm`):
- **SDDM Astronaut Theme** (Animated astronaut theme with elegant QML Wayland animations)
- **Ly** (Ultra-minimalist console TUI matrix login)
- **Greetd (Tuigreet)** (Fast Wayland-native greeter)
- **Manual / TTY** (Direct login from TTY, starts Hyprland on login)

### 5. Waybar Status Bar Layout Variants
Choose status bar styling (cycled with `[B]` or via `--bar`):
- **Floating Island** (Rounded pill islands with window gaps - Aurora signature)
- **Full Width Edge** (Solid edge-to-edge top bar with powerline glyphs)
- **Dual Waybar** (Top status bar + bottom taskbar & workspace dock)
- **Minimal Clean** (Uncluttered time, battery, and workspaces)

### 6. Rofi Application Launcher Styles
Choose launcher style (cycled with `[R]` or via `--launcher`):
- **Spotlight** (Center floating search palette - macOS Spotlight / Raycast feel)
- **Fullscreen Drawer** (Full screen icon grid - GNOME / macOS Launchpad feel)
- **Corner Popup** (Left-aligned discreet desktop launcher)

### 7. Audio & DSP Sound Processing Suite
Choose audio configuration (cycled with `[E]` or via `--audio`):
- **EasyEffects Studio DSP** (PipeWire + EasyEffects with custom equalizer, bass boost, and voice isolation)
- **Pure PipeWire** (Lightweight PipeWire + WirePlumber without DSP effects)

### 8. Kernel & Scheduler Performance Optimizations
Choose kernel process tweaks (cycled with `[Z]` or via `--tweaks`):
- **Ananicy-cpp + ZRAM** (Auto-nice process priority daemon for Hyprland/Games + compressed ZRAM swap)
- **Standard Linux** (Default Arch Linux kernel process scheduling)

### 9. Preferred Terminal Emulator Selection
Choose your default terminal (cycled with `[T]` or via `--terminal`):
- **Kitty** (Aurora signature standard)
- **Ghostty** (Ultra-fast modern GPU-accelerated terminal)
- **Alacritty** (Minimalist high-performance OpenGL terminal)
- **Foot** (Lightweight Wayland native terminal)

### 10. Default Login Shell Selection
Choose your shell environment (cycled with `[H]` or via `--shell`):
- **Fish**: Modern shell with smart autosuggestions, completion, and syntax highlighting.
- **Zsh**: Configured with Starship prompt and high-speed plugins.
- **Bash**: Traditional shell enhanced with Starship prompt.

### 11. Gaming & Performance Suite (`[G]`)
Toggle the dedicated gaming module with `[G]` or `--gaming`:
- Installs **Steam**, **GameMode** (Feral Interactive daemon for CPU/GPU governor priority), **MangoHud** (Vulkan/OpenGL performance overlay), and **Gamescope**.

### 12. Hyprland Animation Speed Profiles (`[O]`)
Cycle animation curves with `[O]`:
- **Snappy** (0.15s instant responsiveness)
- **Smooth** (0.3s fluid cinematic curves)
- **Minimal** (reduced motion for low-latency or battery conservation)

### 13. Live Theme Palette Swatches
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

### Non-Interactive / Headless Mode
Automate installation in scripts or CI:
```bash
./target/release/aurora-installer --non-interactive --preset full --neovim nvchad --terminal ghostty --gaming --theme "Aurora Default"
```

### Command-Line Options

```
Usage: aurora-installer [OPTIONS]

Options:
      --dry-run              Preview actions without applying changes
  -y, --non-interactive      Run without interactive prompts (headless)
  -m, --mode <MODE>          Select release channel (stable vs git)
  -p, --preset <PRESET>      Choose component preset (full, minimal)
      --neovim <NEOVIM>      Choose Neovim preset (lazyvim, nvchad, astronvim, kickstart, none)
      --terminal <TERMINAL>  Choose terminal emulator (kitty, ghostty, alacritty, foot)
      --shell <SHELL>        Choose default shell (fish, zsh, bash)
      --browser <BROWSER>    Choose default browser (zen, brave, firefox, chromium, none)
      --dm <DM>              Choose display manager (sddm, ly, greetd, none)
      --bar <BAR>            Choose Waybar layout (island, full, dual, minimal)
      --launcher <LAUNCHER>  Choose Rofi launcher style (spotlight, drawer, corner)
      --audio <AUDIO>        Choose audio configuration (easyeffects, pipewire)
      --tweaks <TWEAKS>      Choose kernel scheduler tweaks (performance, standard)
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
| **Welcome** | `Enter` | Proceed to Configuration |
| **Welcome** | `Q` / `Esc` | Quit installer |
| **Configuration** | `K` | **Open Granular Package Selector** |
| **Configuration** | `A` | **SELECT ALL PACKAGES GLOBALLY** (in Custom mode) |
| **Configuration** | `X` | **DESELECT ALL PACKAGES GLOBALLY** (in Custom mode) |
| **Configuration** | `V` | **Cycle Neovim Preset** (LazyVim ↔ NvChad ↔ AstroNvim ↔ Kickstart ↔ None) |
| **Configuration** | `T` | **Cycle Terminal Emulator** (Kitty ↔ Ghostty ↔ Alacritty ↔ Foot) |
| **Configuration** | `W` | **Cycle Web Browser** (Zen Browser ↔ Brave ↔ Firefox ↔ Chromium ↔ None) |
| **Configuration** | `B` | **Cycle Waybar Layout** (Floating Island ↔ Full Width ↔ Dual Bar ↔ Minimal) |
| **Configuration** | `R` | **Cycle Rofi Launcher Style** (Spotlight ↔ Fullscreen Drawer ↔ Corner Popup) |
| **Configuration** | `L` | **Cycle Display Manager** (SDDM Astronaut ↔ Ly TUI ↔ Greetd ↔ TTY) |
| **Configuration** | `E` | **Cycle Audio Suite** (EasyEffects Studio DSP ↔ Pure PipeWire) |
| **Configuration** | `Z` | **Cycle Kernel Tweaks** (Ananicy + ZRAM ↔ Standard) |
| **Configuration** | `H` | **Cycle Default Shell** (Fish ↔ Zsh ↔ Bash) |
| **Configuration** | `G` | **Toggle Gaming Suite** (Steam, GameMode, MangoHud) |
| **Configuration** | `O` | **Cycle Animation Curves** (Snappy ↔ Smooth ↔ Minimal) |
| **Configuration** | `P` | Cycle preset (Full Rice ↔ Minimal ↔ Custom) |
| **Configuration** | `M` | Cycle release channel (Stable ↔ Bleeding Edge) |
| **Configuration** | `D` | Toggle Dry-Run safe mode |
| **Configuration** | `S` | Toggle memory-safe sudo-rs |
| **Configuration** | `Tab` / `Shift+Tab` | Cycle active configuration card |
| **Configuration** | `↑` / `↓` | Navigate items in Components checklist or Themes list |
| **Configuration** | `Space` | Toggle component, toggle dry-run, or select theme |
| **Configuration** | `Enter` | Review planned pipeline |
| **Configuration** | `Esc` | Return to Welcome screen |
| **Package Selector** | `Space` | Toggle individual package checkbox |
| **Package Selector** | `A` | **Select All** packages in active category |
| **Package Selector** | `N` | **Deselect All** packages in active category |
| **Package Selector** | `U` / `Shift+A` | **SELECT ALL PACKAGES GLOBALLY** across all categories |
| **Package Selector** | `X` / `Shift+N` | **DESELECT ALL PACKAGES GLOBALLY** across all categories |
| **Package Selector** | `I` | **Invert** selection in active category |
| **Package Selector** | `Shift+I` | **Invert** selection across all categories |
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
