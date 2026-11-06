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
*Native support for NixOS, Fedora and Fedora-based distributions, and Arch Linux and Arch-based distributions.*

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with Ratatui](https://img.shields.io/badge/TUI-Ratatui%200.30-cyan.svg)](https://ratatui.rs)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-orange.svg)](https://www.rust-lang.org)

</div>

---

## ✦ Overview

The **Aurora TUI Installer** (`aurora-installer`) provides a responsive, keyboard-driven terminal user interface built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).

Aurora delivers a unified, first-class installation pipeline across three major Linux distribution ecosystems:
1. **NixOS**: Fully declarative setup with Flakes (`flake.nix`, `home.nix`, `configuration.nix`), dev shell build environments, SDDM astronaut theme packaging, and Zen Browser Flatpak integration.
2. **Fedora & Fedora-based Distributions**: Tailored for Fedora Workstation, Silverblue, Kinoite, Sericea, Onyx, Nobara Linux, Bazzite, Ultramarine Linux, RHEL, CentOS Stream, Rocky Linux, and AlmaLinux via DNF, COPR repositories, and automated source compilation.
3. **Arch Linux & Arch-based Distributions**: Complete support for Arch Linux, CachyOS, EndeavourOS, Manjaro, Garuda, Artix, ArcoLinux, Archcraft, ChimeraOS, BlendOS, RebornOS, SteamOS, and all Arch derivatives via `pacman` and AUR helpers (`yay` / `paru`).

---

## ✦ Distribution Support & Detection

### Supported Distribution Matrix

| Distribution Family | Package System | Supported Flavors & Derivatives |
|---|---|---|
| **Arch Linux** | `pacman`, `yay`, `paru` (AUR) | Pure Arch Linux, CachyOS, EndeavourOS, Manjaro, Garuda Linux, Artix Linux, ArcoLinux, Archcraft, ChimeraOS, BlendOS, RebornOS, SteamOS, and all systems with `pacman` or `arch` in `ID_LIKE`. |
| **Fedora** | `dnf`, COPR repositories | Fedora Workstation, Fedora Silverblue, Kinoite, Sericea, Onyx, Nobara Linux, Bazzite, Ultramarine Linux, RHEL, CentOS Stream, Rocky Linux, AlmaLinux. |
| **NixOS** | `nix` (Flakes), `nixos-rebuild`, Flatpak | NixOS 24.05+, NixOS Unstable with declarative Nix flakes and Home Manager configurations. |

### Automatic Detection & Manual Override
- **Automatic Detection**: On startup, the installer inspects `/etc/os-release`, checks available package management binaries (`pacman`, `dnf`, `nix`), and verifies system architecture and GPU drivers.
- **Interactive Switching**: Press `[D]` on the **Welcome screen** to cycle target distributions between **Arch Linux**, **Fedora**, and **NixOS**.
- **CLI Flag Override**: Pass `--distro arch`, `--distro fedora`, or `--distro nixos` to explicitly target a specific distribution family.

---

## ✦ Distro-Specific Pipelines

### 1. Arch Linux Pipeline
- **Package Management**: Validates `pacman` and AUR helper (`yay` or `paru`), automatically bootstrapping `yay` if no AUR helper is present.
- **Categorized Installation**: Installs packages across Core Hyprland, System Daemons, UI/Rice Elements, Utilities, Development/Build tools, and AUR Extras.
- **Network Management**: Disables conflicting network daemons (`iwd`, `systemd-networkd`, `connman`, `dhcpcd`, `wicd`) and enables `NetworkManager.service`.
- **SDDM Astronaut Display Manager**: Deploys the SDDM theme, copies required fonts to `/usr/share/fonts`, updates font cache (`fc-cache -f`), enables `sddm.service`, and configures `/etc/sddm.conf` and `/etc/sddm.conf.d/virtualkbd.conf`.
- **Preserved User Configuration**: Deploys Aurora dotfiles while strictly preserving `~/.config/hypr/User`.
- **Toolchains & Themes**: Bootstraps Rust tools (`swaylock-effects`, `waytrogen`), configures shell environments (`.zshrc`, `.bashrc`, `.bash_profile`, `config.fish`), and applies active theme via `awww-daemon` and `waybar`.

### 2. Fedora Pipeline
- **COPR Repositories**: Automatically enables required COPR repositories:
  - `solopasha/hyprland` & `lionheartp/Hyprland` (Hyprland ecosystem)
  - `sneexy/zen-browser` (Zen Browser)
  - `maveonair/jetbrains-mono-nerd-fonts` (JetBrains Mono Nerd Font)
  - `atim/starship` (Starship prompt)
  - `erikreider/swayosd` (SwayOSD on-screen display)
  - `paolino/hyprmoncfg` (Hyprland monitor config utility)
- **DNF Package Installation**: Installs comprehensive Fedora package suite (Wayland libraries, Qt6, audio, tools).
- **Source Compilation**: Automatically compiles and installs tools not natively packaged in Fedora:
  - `hyprwave`, `Nordzy-cursors`, `rofi-emoji`, `snappy-switcher`, `waytrogen-aurora`
- **Display Manager Migration**: Disables competing display managers (`gdm`, `lightdm`, `lxdm`, `kdm`) and activates `sddm.service`.
- **Preserved Dotfiles & Shells**: Preserves `~/.config/hypr/User` and updates shell environment configs.

### 3. NixOS Pipeline
- **Declarative Nix Flakes**: Enables experimental features (`nix-command flakes`).
- **Configuration Deployment**: Deploys NixOS flake files (`configuration.nix`, `home.nix`, `flake.nix`, `flake.lock`, `hardware-configuration.nix`, `variables.nix`) to `/etc/nixos` (cloning from upstream repository if not present locally).
- **System Rebuild**: Executes `sudo nixos-rebuild switch --flake /etc/nixos#nixos`.
- **Nix Dev Shell Toolchain**: Builds Rust utilities and `waytrogen` within `nix develop /etc/nixos -c cargo install`.
- **Flatpak Integration**: Automatically configures Flatpak Flathub repository and installs Zen Browser (`io.github.zen_browser.zen`).
- **SDDM Astronaut Packaging**: Installs and configures SDDM astronaut theme declaratively.
- **Preserved Dotfiles**: Preserves `~/.config/hypr/User` during dotfiles sync.

---

## ✦ Dotfiles Safety & Preservation

> [!IMPORTANT]
> **Safety Guarantee**: The Aurora installer **never** overwrites or deletes your personal custom configurations.
> The directory `~/.config/hypr/User` is always backed up and preserved across all distributions.
> Dotfiles backup strategies can also be configured to timestamped backups (`~/.config.aurora_bak.<timestamp>`), direct overwrite with preservation, or skipping existing files.

---

## ✦ Extraordinary Customization Options

### 1. Granular Package Selector & Bulk Controls
Fine-tune and customize the exact packages installed, with instant bulk controls:
- **Global Bulk Controls**:
  - `[U]` or `[Shift+A]`: **SELECT ALL PACKAGES GLOBALLY** (across all categories)
  - `[X]` or `[Shift+N]`: **DESELECT ALL PACKAGES GLOBALLY** (start from a blank slate to pick only what you need)
  - `[Shift+I]`: **INVERT SELECTION GLOBALLY**
  - In Configuration Preset card: Press `[A]` to Enable All, `[X]` to Disable All.
- **Category-Level Shortcuts (`[K]` Package Customizer)**:
  - `[/]`: Live search filter across package names
  - `[Space]`: Toggle individual package checkbox
  - `[A]`: Select All packages in active category or filtered search
  - `[N]`: Deselect All packages in active category or filtered search
  - `[I]`: Invert selection in active category or filtered search
  - `[Tab] / [←/→]`: Switch between Categories and Packages column
  - `[Enter] / [Esc] / [B]`: Save selection and return to Configuration

### 2. Distribution Selection (`[D]` on Welcome Screen / `--distro`)
- Cycle between **Arch Linux**, **Fedora**, and **NixOS** directly from the Welcome screen.

### 3. Neovim IDE Distribution Selection (`[V]` / `--neovim`)
- **LazyVim Starter** (Aurora official default): Complete preconfigured Neovim environment with Lazy.nvim, LSP, Treesitter, and Which-Key.
- **NvChad**: Blazing fast startup time (<20ms) with rich UI popups, statusline themes, and cheatsheet.
- **AstroNvim**: Feature-rich extensible configuration tailored for modern full-stack development.
- **Kickstart.nvim**: Single-file modular template documented for clean personal customization.
- **None / Keep Existing**: Preserves your existing `~/.config/nvim` untouched.

### 4. Default Web Browser Selection (`[W]` / `--browser`)
- **Zen Browser** (Aurora Signature - vertical split tabs, Arc-like workflow, privacy-first)
- **Brave** (Native adblocking & privacy shield)
- **Firefox** (Official Wayland open-source standard)
- **Chromium** (Pure engine & high compatibility)
- **None** (Preserve current browser untouched)

### 5. Display Manager / Login Screen Selection (`[L]` / `--dm`)
- **SDDM Astronaut Theme** (Animated astronaut theme with elegant QML Wayland animations)
- **Ly** (Ultra-minimalist console TUI matrix login)
- **Greetd (Tuigreet)** (Fast Wayland-native greeter)
- **Manual / TTY** (Direct login from TTY, starts Hyprland on login)

### 6. Waybar Status Bar Layout Variants (`[B]` / `--bar`)
- **Floating Island** (Rounded pill islands with window gaps - Aurora signature)
- **Full Width Edge** (Solid edge-to-edge top bar with powerline glyphs)
- **Dual Waybar** (Top status bar + bottom taskbar & workspace dock)
- **Minimal Clean** (Uncluttered time, battery, and workspaces)

### 7. Rofi Application Launcher Styles (`[R]` / `--launcher`)
- **Spotlight** (Center floating search palette - macOS Spotlight / Raycast feel)
- **Fullscreen Drawer** (Full screen icon grid - GNOME / macOS Launchpad feel)
- **Corner Popup** (Left-aligned discreet desktop launcher)

### 8. Audio & DSP Sound Processing Suite (`[E]` / `--audio`)
- **EasyEffects Studio DSP** (PipeWire + EasyEffects with custom equalizer, bass boost, and voice isolation)
- **Pure PipeWire** (Lightweight PipeWire + WirePlumber without DSP effects)

### 9. Kernel & Scheduler Performance Optimizations (`[Z]` / `--tweaks`)
- **Ananicy-cpp + ZRAM** (Auto-nice process priority daemon for Hyprland/Games + compressed ZRAM swap)
- **Standard Linux** (Default kernel process scheduling)

### 10. Preferred Terminal Emulator Selection (`[T]` / `--terminal`)
- **Kitty** (Aurora signature standard)
- **Ghostty** (Ultra-fast modern GPU-accelerated terminal)
- **Alacritty** (Minimalist high-performance OpenGL terminal)
- **Foot** (Lightweight Wayland native terminal)

### 11. Monospace Nerd Font Choice (`[F]` / `--font`)
- **JetBrains Mono Nerd Font** (Aurora signature coding font)
- **Fira Code Nerd Font** (Ligature-rich programming font)
- **Cascadia Code Nerd Font** (Modern Microsoft typography)
- **Hack Nerd Font** (Clean, legible monospace standard)

### 12. Wayland Lockscreen Daemon (`[C]` / `--lockscreen`)
- **Hyprlock** (Fast, elegant Wayland lockscreen for Hyprland)
- **Swaylock-effects** (Swaylock fork with blur, fade, and indicator effects)
- **None** (Do not install or configure a lockscreen)

### 13. GPU Driver Profile (`[Y]` / `--gpu`)
- **Auto-Detect** (Inspects hardware via `sysinfo` / `lspci` / `/sys/class/drm`)
- **NVIDIA Proprietary** (Configures `nvidia_drm.modeset=1`, Wayland environment flags)
- **AMD Mesa** (Configures Radv Vulkan driver and Mesa Wayland acceleration)
- **Intel Mesa** (Configures Intel Iris / ANV Vulkan driver)
- **Virtual Machine** (Disables hardware acceleration flags for VirtualBox / QEMU / VMware)

### 14. Wallpaper Engine Daemon (`[J]` / `--wallpaper`)
- **Swww** (Smooth animated wallpaper daemon)
- **Hyprpaper** (Fast, native Hyprland wallpaper utility)
- **Awww** (Lightweight wallpaper daemon)
- **Mpvpaper** (Video wallpaper daemon backed by MPV)

### 15. Default Login Shell Selection (`[H]` / `--shell`)
- **Fish**: Modern shell with smart autosuggestions, completion, and syntax highlighting.
- **Zsh**: Configured with Starship prompt and high-speed plugins.
- **Bash**: Traditional shell enhanced with Starship prompt.

### 16. Gaming & Performance Suite (`[G]` / `--gaming`)
Toggle the dedicated gaming module with `[G]` or `--gaming`:
- Installs **Steam**, **GameMode** (Feral Interactive daemon for CPU/GPU governor priority), **MangoHud** (Vulkan/OpenGL performance overlay), and **Gamescope**.

### 17. Hyprland Animation Speed Profiles (`[O]`)
Cycle animation curves with `[O]`:
- **Snappy** (0.15s instant responsiveness)
- **Smooth** (0.3s fluid cinematic curves)
- **Minimal** (reduced motion for low-latency or battery conservation)

### 18. Live Theme Palette Swatches
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

### Distro Override
Override detection to preview or install for a specific target distribution:
```bash
# Arch Linux and derivatives
./target/release/aurora-installer --distro arch

# Fedora and derivatives
./target/release/aurora-installer --distro fedora

# NixOS
./target/release/aurora-installer --distro nixos
```

### Dry-Run Preview Mode
Preview all commands, configurations, and scripts safely without modifying your system:
```bash
./target/release/aurora-installer --dry-run
```

### Hardware & System Preflight Diagnostics
Run hardware, kernel, memory, storage, and GPU audits:
```bash
./target/release/aurora-installer --diagnostics
```

### Non-Interactive / Headless Mode
Automate installation in scripts or CI:
```bash
./target/release/aurora-installer --distro arch --non-interactive --preset full --neovim lazyvim --terminal kitty --gaming --theme "Aurora Default"
```

### Export & Import Configuration Profiles
Export your configured setup to a reusable JSON profile or load an existing one:
```bash
# Export
./target/release/aurora-installer --export-profile ~/my-aurora-profile.json

# Import
./target/release/aurora-installer --profile ~/my-aurora-profile.json --non-interactive
```

---

## ✦ Command-Line Options Reference

```
Usage: aurora-installer [OPTIONS]

Options:
  -D, --distro <DISTRO>          Target distribution family (arch, fedora, nixos)
      --dry-run                  Preview actions without applying changes
  -y, --non-interactive          Run without interactive prompts (headless)
  -m, --mode <MODE>              Select release channel (stable vs git)
  -p, --preset <PRESET>          Choose component preset (full, minimal)
      --neovim <NEOVIM>          Choose Neovim preset (lazyvim, nvchad, astronvim, kickstart, none)
      --terminal <TERMINAL>      Choose terminal emulator (kitty, ghostty, alacritty, foot)
      --shell <SHELL>            Choose default shell (fish, zsh, bash)
      --browser <BROWSER>        Choose default browser (zen, brave, firefox, chromium, none)
      --dm <DM>                  Choose display manager (sddm, ly, greetd, none)
      --bar <BAR>                Choose Waybar layout (island, full, dual, minimal)
      --launcher <LAUNCHER>      Choose Rofi launcher style (spotlight, drawer, corner)
      --audio <AUDIO>            Choose audio configuration (easyeffects, pipewire)
      --tweaks <TWEAKS>          Choose kernel scheduler tweaks (performance, standard)
      --font <FONT>              Choose monospace font (jetbrains, fira, cascadia, hack)
      --lockscreen <LOCKSCREEN>  Choose lockscreen daemon (hyprlock, swaylock, none)
      --gpu <GPU>                Choose GPU driver profile (auto, nvidia, amd, intel, vm)
      --wallpaper <WALLPAPER>    Choose wallpaper engine (swww, hyprpaper, awww, mpvpaper)
      --backup <BACKUP>          Choose backup strategy (timestamp, overwrite, skip)
      --export-profile <PATH>    Export configuration to JSON file
      --profile <PATH>           Load configuration from JSON file
      --diagnostics              Run preflight hardware diagnostics check and exit
      --packages <PACKAGES>      Comma-separated explicit package list to install
      --exclude-packages <LIST>  Comma-separated package list to exclude
      --gaming                   Enable gaming suite (Steam, GameMode, MangoHud)
  -t, --theme <THEME>            Choose default Aurora theme
      --dir <DIR>                Specify local Aurora dotfiles checkout directory
      --no-sudo-rs               Do not switch sudo to sudo-rs
      --uninstall                Uninstall Aurora configurations and restore backups
      --debug                    Enable verbose debug logging
  -h, --help                     Print help
  -V, --version                  Print version
```

---

## ✦ TUI Navigation & Keybindings

| Screen | Keybinding | Action |
|---|---|---|
| **All Screens** | `Ctrl + C` | Emergency exit |
| **All Screens** | `Q` | Exit installer (when not typing in search) |
| **Welcome** | `Enter` | Proceed to Configuration |
| **Welcome** | `D` | **Cycle Target Distribution** (Arch ↔ Fedora ↔ NixOS) |
| **Welcome** | `I` | Open Hardware & Diagnostics preflight report |
| **Welcome** | `Q` / `Esc` | Quit installer |
| **Configuration** | `K` | **Open Granular Package Selector** |
| **Configuration** | `A` | **SELECT ALL PACKAGES GLOBALLY** |
| **Configuration** | `X` | **DESELECT ALL PACKAGES GLOBALLY** |
| **Configuration** | `P` | Cycle Preset (Full Rice ↔ Minimal ↔ Custom) |
| **Configuration** | `V` | **Cycle Neovim Preset** (LazyVim ↔ NvChad ↔ AstroNvim ↔ Kickstart ↔ None) |
| **Configuration** | `T` | **Cycle Terminal Emulator** (Kitty ↔ Ghostty ↔ Alacritty ↔ Foot) |
| **Configuration** | `W` | **Cycle Web Browser** (Zen Browser ↔ Brave ↔ Firefox ↔ Chromium ↔ None) |
| **Configuration** | `B` | **Cycle Waybar Layout** (Floating Island ↔ Full Width ↔ Dual Bar ↔ Minimal) |
| **Configuration** | `R` | **Cycle Rofi Launcher Style** (Spotlight ↔ Fullscreen Drawer ↔ Corner Popup) |
| **Configuration** | `L` | **Cycle Display Manager** (SDDM Astronaut ↔ Ly TUI ↔ Greetd ↔ TTY) |
| **Configuration** | `C` | **Cycle Lockscreen Daemon** (Hyprlock ↔ Swaylock-effects ↔ None) |
| **Configuration** | `F` | **Cycle Monospace Font** (JetBrains Mono ↔ Fira Code ↔ Cascadia ↔ Hack) |
| **Configuration** | `J` | **Cycle Wallpaper Engine** (Swww ↔ Hyprpaper ↔ Awww ↔ Mpvpaper) |
| **Configuration** | `Y` | **Cycle GPU Driver Profile** (Auto-Detect ↔ NVIDIA ↔ AMD ↔ Intel ↔ VM) |
| **Configuration** | `U` | **Cycle Dotfiles Backup Strategy** (Timestamped ↔ Direct Overwrite ↔ Skip) |
| **Configuration** | `E` | **Cycle Audio Suite** (EasyEffects Studio DSP ↔ Pure PipeWire) |
| **Configuration** | `Z` | **Cycle Kernel Tweaks** (Ananicy-cpp + ZRAM ↔ Standard) |
| **Configuration** | `H` | **Cycle Default Shell** (Fish ↔ Zsh ↔ Bash) |
| **Configuration** | `G` | **Toggle Gaming Suite** (Steam, GameMode, MangoHud) |
| **Configuration** | `O` | **Cycle Animation Curves** (Snappy ↔ Smooth ↔ Minimal) |
| **Configuration** | `M` | Cycle Release Channel (Stable ↔ Bleeding-Edge Git) |
| **Configuration** | `D` | Toggle Dry-Run safe mode |
| **Configuration** | `S` | Toggle memory-safe sudo-rs |
| **Configuration** | `I` | View System Diagnostics & Preflight check |
| **Configuration** | `Tab` / `BackTab` | Cycle active configuration card section |
| **Configuration** | `↑` / `↓` | Navigate items in Components checklist or Themes list |
| **Configuration** | `Space` | Toggle component, toggle dry-run, or select theme |
| **Configuration** | `Enter` | Review planned pipeline in Summary screen |
| **Configuration** | `Esc` | Return to Welcome screen |
| **Package Selector** | `/` | **Search & Filter packages** by name |
| **Package Selector** | `Space` | Toggle individual package checkbox |
| **Package Selector** | `A` | **Select All** packages in active category / filter |
| **Package Selector** | `N` | **Deselect All** packages in active category / filter |
| **Package Selector** | `I` | **Invert** selection in active category / filter |
| **Package Selector** | `U` / `Shift+A` | **SELECT ALL PACKAGES GLOBALLY** across all categories |
| **Package Selector** | `X` / `Shift+N` | **DESELECT ALL PACKAGES GLOBALLY** across all categories |
| **Package Selector** | `Shift+I` | **INVERT ALL PACKAGES GLOBALLY** across all categories |
| **Package Selector** | `Tab` / `←` / `→` | Switch between Categories and Packages column |
| **Package Selector** | `Enter` / `Esc` / `B` | Save selection and return to Configuration |
| **Diagnostics** | `Esc` / `Enter` / `I` | Return to previous screen |
| **Summary** | `Enter` / `Y` | **Start installation pipeline** |
| **Summary** | `Esc` / `B` | Go back and adjust configuration |
| **Summary** | `Q` | Cancel and quit |
| **Installation** | `S` | Toggle live log auto-scroll |
| **Installation** | `↑` / `↓` / `PgUp` / `PgDn` | Scroll terminal log history |
| **Complete** | `Enter` / `Q` | Exit installer |
| **Complete** | `R` | Reboot system |

---

## ✦ License

This project is licensed under the **GNU General Public License v3.0 or later** ([GPL-3.0-or-later](LICENSE)).

Copyright (C) 2026 Ahum Maitra <theahummaitra@gmail.com>
