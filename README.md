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
- **Automatic Detection**: On startup, the installer inspects `/etc/os-release`, checks available package management binaries (`pacman`, `dnf`, `nix`), checks laptop chassis vs desktop hardware, detects Btrfs filesystem support, and verifies GPU drivers.
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

## ✦ Extraordinary Features Suite

### 1. Aurora System Doctor & Health Verifier (`--doctor` / `[V]`)
Run an instant 8-subsystem diagnostic audit verifying:
- Hyprland binary and syntax integrity.
- Waybar configuration and CSS stylesheet validity.
- PipeWire, WirePlumber, and physical ALSA audio cards.
- Display manager, SDDM greeter, and theme availability.
- Dotfiles preservation check (guarantees `~/.config/hypr/User` integrity).
- Direct rendering DRM/KMS graphics acceleration nodes.
- High-resolution monospace Nerd Font cache status.
- Wayland wallpaper engine daemon responsiveness.

```bash
# Run standalone health verification anytime
./target/release/aurora-installer --doctor
```

### 2. Specialized Installation Presets (`[P]` / `--preset`)
Cycle between 5 tailored presets designed for specific workflows:
- **Full Rice**: Complete desktop suite with SDDM Astronaut theme, Neovim IDE, tools, fonts, and dotfiles.
- **Minimal Core**: Lightweight foundation containing only Hyprland, Waybar, core dotfiles, and essential utils.
- **Gaming & Steam Rice**: Optimized for low latency with Steam, GameMode, Proton-GE, MangoHud, Performance governor, and Snappy animation curves.
- **OLED & Battery Laptop Rice**: Battery longevity profile with true pitch-black `#000000` backgrounds, 0% blur, TLP power-saving, and 80% charge threshold.
- **Custom Selection**: Fine-grained component selection from scratch.

### 3. Hyprland Window Effects & Blur Presets (`[2]` / `--effects`)
Choose from 4 preconfigured Hyprland window decoration and blur shader profiles:
- **Frosted Glass**: Heavy Gaussian blur (size 8, passes 3), subtle drop shadow, 12px rounded borders, 0.90 opacity — *Aurora signature look*.
- **Cyberpunk Neon**: Glowing dual gradient active borders (`#ff007f` to `#00f0ff`), vivid drop shadow, 0.95 opacity, snappy motion.
- **OLED Pure Black**: #000000 true pitch-black backgrounds, 100% opacity, zero blur, max power savings on OLED displays.
- **Minimal Flat**: Sharp 0px corners, zero blur, 1px subtle borders, ultra-low GPU overhead.

### 4. Laptop Power & Battery Care Suite (`[1]` / `--power`)
Intelligently detects battery presence and provides 4 power profiles:
- **Balanced**: Standard Linux CPU governor (ondemand/schedutil) with balanced display timeouts.
- **Performance Boost**: AC desktop & gaming mode: maximum CPU boost, performance governor, PCIe ASPM disabled, no display sleep.
- **Battery Care**: Laptop longevity: auto-cpufreq / power-profiles-daemon power-saving, 80% charge limit on ASUS/ThinkPad/Lenovo hardware, 3m screen dimming, and suspend on lid close.
- **None**: Keeps existing system power configurations untouched.

### 5. Desktop Security & Privacy Hardening Suite (`[5]`, `[6]`, `[0]`)
- **Desktop Firewall (`[5]` / `--firewall`)**:
  - `UFW Desktop Hardened`: Default deny incoming, allow outgoing, with preconfigured LAN pinholes for LocalSend (port 53317), KDE Connect (ports 1714–1764), and mDNS (port 5353).
  - `Firewalld`: Configures Fedora standard desktop zone.
  - `None`: Leaves existing firewall untouched.
- **Privacy DNS Profile (`[6]` / `--dns`)**:
  - `Quad9`: 9.9.9.9 / 149.112.112.112 with DNS-over-TLS (threat blocking & zero logging).
  - `Cloudflare`: 1.1.1.1 / 1.0.0.1 (low latency DoH).
  - `AdGuard`: 94.140.14.14 (network-wide ad & tracker blocking).
  - `SystemDefault`: System default ISP/router DNS.
- **MAC Address Randomization (`[0]` / `--mac-randomize`)**:
  - Automatically randomizes WiFi and Ethernet MAC addresses on connection for public network anonymity.

### 6. Cursor & Icon Themes Customizer (`[3]`, `[4]`)
- **Cursor Themes (`[3]` / `--cursor`)**:
  - **Nordzy Cursors** (Aurora Signature Nordic design)
  - **Bibata Modern Classic** (Minimalist smooth rounded cursors)
  - **Catppuccin Mocha Cursors** (Pastel aesthetic)
  - **Breeze** (Clean desktop standard)
- **Icon Themes (`[4]` / `--icons`)**:
  - **Papirus Dark** (Aurora official vector icon set)
  - **Tela Circle** (Modern circular colorful icons)
  - **WhiteSur** (macOS-inspired clean rounded icons)
  - **Fluent Dark** (Microsoft Fluent design language)

### 7. Virtualization & Containerization Module (`[7]` / `--virt`)
- **QEMU / KVM + Virt-Manager**: Enables `libvirtd.service`, autostarts default NAT network, and adds user to `libvirt` and `kvm` groups.
- **Docker + Podman Rootless**: Enables container runtime services with rootless user group permissions.
- **None**: No virtualization daemons.

### 8. Btrfs Pre-Install Instant Rollback Snapshot (`[8]` / `--no-snapshot`)
- Automatically detects if `/` or `/home` is on a Btrfs filesystem.
- Generates a read-only snapshot (`/.aurora_pre_install_snapshot_<timestamp>`) before any package is installed or dotfile is touched.
- Enables instant 1-second system rollback if needed.

### 9. Automated Weekly Maintenance Timers (`[9]` / `--no-maintenance`)
- Installs `aurora-maintenance.timer` under systemd user services to run weekly:
  - Cleans package caches (`paccache -rk2` on Arch, `dnf clean packages` on Fedora, `nix-collect-garbage -d` on NixOS).
  - Performs SSD fstrim.
  - Generates weekly timestamped dotfiles backups.

### 10. Steam & Proton-GE Gaming Enhancements (`--proton-ge`)
- Auto-configures Proton-GE compatibility directories (`~/.steam/root/compatibilitytools.d/`).
- Deploys custom MangoHud gaming overlay metrics (`~/.config/MangoHud/MangoHud.conf`).
- Configures GameMode CPU governor priorities.

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

### System Health Doctor Verification
Run instant preflight and post-install health verification:
```bash
./target/release/aurora-installer --doctor
```

### Distro Override
Preview or install for a specific target distribution:
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

### Seamless Sudo Privilege Escalation & Keepalive
System-level operations (package installations, systemd services, display managers) require `sudo` privileges. The installer provides seamless privilege escalation:
- **Clean Terminal Password Prompt**: If sudo credentials are not already cached, the installer cleanly suspends the TUI upon confirmation, prompts for your password in your standard terminal, verifies it with `sudo -v`, and seamlessly resumes the TUI.
- **Background Sudo Keepalive**: During the entire installation pipeline, a background worker automatically refreshes the sudo timestamp (`sudo -v -n`) every 30 seconds, preventing credentials from expiring during long-running builds or downloads.
- **Non-Interactive Mode Support**: When running with `--non-interactive`, sudo credentials are pre-verified in the terminal before execution starts.
- **Dry-Run Safe**: In `--dry-run` or root user mode, privilege escalation is automatically bypassed.

### Hardware & System Preflight Diagnostics
Run hardware, kernel, memory, storage, laptop battery, and GPU audits:
```bash
./target/release/aurora-installer --diagnostics
```

### Specialized Non-Interactive Presets
Automate installation in scripts or CI:
```bash
# Gaming-optimized setup
./target/release/aurora-installer --preset gaming --non-interactive

# OLED Laptop battery-saving setup
./target/release/aurora-installer --preset oled --non-interactive
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
  -p, --preset <PRESET>          Choose preset (full, minimal, gaming, oled)
      --effects <EFFECTS>        Choose window effects (frosted, neon, oled, flat)
      --power <POWER>            Choose power profile (balanced, performance, battery, none)
      --firewall <FIREWALL>      Choose firewall (ufw, firewalld, none)
      --dns <DNS>                Choose privacy DNS (quad9, cloudflare, adguard, system)
      --mac-randomize            Enable WiFi & Ethernet MAC randomization
      --cursor <CURSOR>          Choose cursor theme (nordzy, bibata, catppuccin, breeze)
      --icons <ICONS>            Choose icon theme (papirus, tela, whitesur, fluent)
      --virt <VIRT>              Choose virtualization suite (qemu, docker, none)
      --no-snapshot              Skip pre-installation Btrfs rollback snapshot
      --no-maintenance           Skip automated weekly maintenance timers
      --proton-ge                Configure Proton-GE & MangoHud gaming overlay
      --doctor                   Run system doctor health verification and exit
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
| **Welcome** | `V` | **Run System Doctor** health verification check |
| **Welcome** | `Q` / `Esc` | Quit installer |
| **Configuration** | `P` | **Cycle Preset** (Full Rice ↔ Minimal ↔ Gaming ↔ OLED Laptop ↔ Custom) |
| **Configuration** | `1` | **Cycle Power Profile** (Balanced ↔ Performance ↔ Battery Care ↔ None) |
| **Configuration** | `2` | **Cycle Window Effects** (Frosted Glass ↔ Cyberpunk Neon ↔ OLED Pure Black ↔ Minimal Flat) |
| **Configuration** | `3` | **Cycle Cursor Theme** (Nordzy ↔ Bibata Modern ↔ Catppuccin Mocha ↔ Breeze) |
| **Configuration** | `4` | **Cycle Icon Theme** (Papirus Dark ↔ Tela Circle ↔ WhiteSur ↔ Fluent Dark) |
| **Configuration** | `5` | **Cycle Firewall** (UFW Desktop Hardened ↔ Firewalld ↔ None) |
| **Configuration** | `6` | **Cycle Privacy DNS** (Quad9 9.9.9.9 ↔ Cloudflare 1.1.1.1 ↔ AdGuard ↔ System) |
| **Configuration** | `7` | **Cycle Virtualization** (None ↔ QEMU/KVM + Virt-Manager ↔ Docker + Podman) |
| **Configuration** | `8` | **Toggle Btrfs Instant Rollback Snapshot** |
| **Configuration** | `9` | **Toggle Automated Maintenance Timers** |
| **Configuration** | `0` | **Toggle MAC Address Randomization** |
| **Configuration** | `K` | **Open Granular Package Selector** |
| **Configuration** | `A` | **SELECT ALL PACKAGES GLOBALLY** |
| **Configuration** | `X` | **DESELECT ALL PACKAGES GLOBALLY** |
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
| **Complete** | `V` | **Run System Doctor** health verification audit |
| **Complete** | `Enter` / `Q` | Exit installer |
| **Complete** | `R` | Reboot system |

---

## ✦ License

This project is licensed under the **GNU General Public License v3.0 or later** ([GPL-3.0-or-later](LICENSE)).

Copyright (C) 2026 Ahum Maitra <theahummaitra@gmail.com>
