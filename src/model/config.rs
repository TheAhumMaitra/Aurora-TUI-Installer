// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallMode {
    Stable,
    Git,
}

#[allow(dead_code)]
impl InstallMode {
    pub fn label(&self) -> &'static str {
        match self {
            InstallMode::Stable => "Stable (Recommended)",
            InstallMode::Git => "Bleeding Edge (Git / Master)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallMode::Stable => "Uses official, well-tested package releases from distributions",
            InstallMode::Git => "Builds or fetches latest git master packages (hyprland-git, etc.)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallPreset {
    FullRice,
    MinimalRice,
    Custom,
}

impl InstallPreset {
    pub fn label(&self) -> &'static str {
        match self {
            InstallPreset::FullRice => "Full Rice (Recommended)",
            InstallPreset::MinimalRice => "Minimal Core",
            InstallPreset::Custom => "Custom Selection",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallPreset::FullRice => "Installs everything: desktop, SDDM astronaut theme, Waytrogen, Neovim, tools & fonts",
            InstallPreset::MinimalRice => "Installs only core Hyprland, Waybar, basic dotfiles and essential utils",
            InstallPreset::Custom => "Fine-tune individual components, package lists, and development presets",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeovimPreset {
    LazyVim,
    NvChad,
    AstroNvim,
    Kickstart,
    None,
}

#[allow(dead_code)]
impl NeovimPreset {
    pub const ALL: &[NeovimPreset] = &[
        NeovimPreset::LazyVim,
        NeovimPreset::NvChad,
        NeovimPreset::AstroNvim,
        NeovimPreset::Kickstart,
        NeovimPreset::None,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            NeovimPreset::LazyVim => "LazyVim Starter (Aurora Official)",
            NeovimPreset::NvChad => "NvChad (Blazing Fast & Beautiful)",
            NeovimPreset::AstroNvim => "AstroNvim (Aesthetic Extensible IDE)",
            NeovimPreset::Kickstart => "Kickstart.nvim (Modular & Minimal)",
            NeovimPreset::None => "None / Keep Existing Configuration",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            NeovimPreset::LazyVim => "Preconfigured Neovim with Lazy.nvim, LSP, treesitter, fuzzy finder & which-key",
            NeovimPreset::NvChad => "Blazing fast startup time (<20ms) with rich UI popups and custom statusline",
            NeovimPreset::AstroNvim => "Feature-rich extensible configuration tailored for modern full-stack development",
            NeovimPreset::Kickstart => "Single-file modular starting point documented for personal customization",
            NeovimPreset::None => "Do not touch ~/.config/nvim; preserve current user Neovim setup",
        }
    }

    pub fn repo_url(&self) -> Option<&'static str> {
        match self {
            NeovimPreset::LazyVim => Some("https://github.com/LazyVim/starter"),
            NeovimPreset::NvChad => Some("https://github.com/NvChad/starter"),
            NeovimPreset::AstroNvim => Some("https://github.com/AstroNvim/template"),
            NeovimPreset::Kickstart => Some("https://github.com/nvim-lua/kickstart.nvim"),
            NeovimPreset::None => None,
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            NeovimPreset::LazyVim => NeovimPreset::NvChad,
            NeovimPreset::NvChad => NeovimPreset::AstroNvim,
            NeovimPreset::AstroNvim => NeovimPreset::Kickstart,
            NeovimPreset::Kickstart => NeovimPreset::None,
            NeovimPreset::None => NeovimPreset::LazyVim,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalChoice {
    Kitty,
    Ghostty,
    Alacritty,
    Foot,
}

#[allow(dead_code)]
impl TerminalChoice {
    pub const ALL: &[TerminalChoice] = &[
        TerminalChoice::Kitty,
        TerminalChoice::Ghostty,
        TerminalChoice::Alacritty,
        TerminalChoice::Foot,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            TerminalChoice::Kitty => "Kitty (Aurora Default)",
            TerminalChoice::Ghostty => "Ghostty (GPU Accelerated)",
            TerminalChoice::Alacritty => "Alacritty (Simplicity & Speed)",
            TerminalChoice::Foot => "Foot (Lightweight Wayland Native)",
        }
    }

    pub fn package_name(&self) -> &'static str {
        match self {
            TerminalChoice::Kitty => "kitty",
            TerminalChoice::Ghostty => "ghostty",
            TerminalChoice::Alacritty => "alacritty",
            TerminalChoice::Foot => "foot",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            TerminalChoice::Kitty => TerminalChoice::Ghostty,
            TerminalChoice::Ghostty => TerminalChoice::Alacritty,
            TerminalChoice::Alacritty => TerminalChoice::Foot,
            TerminalChoice::Foot => TerminalChoice::Kitty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellChoice {
    Fish,
    Zsh,
    Bash,
}

#[allow(dead_code)]
impl ShellChoice {
    pub const ALL: &[ShellChoice] = &[ShellChoice::Fish, ShellChoice::Zsh, ShellChoice::Bash];

    pub fn label(&self) -> &'static str {
        match self {
            ShellChoice::Fish => "Fish (Smart Autosuggestions)",
            ShellChoice::Zsh => "Zsh + Starship Prompt",
            ShellChoice::Bash => "Bash + Starship Prompt",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            ShellChoice::Fish => ShellChoice::Zsh,
            ShellChoice::Zsh => ShellChoice::Bash,
            ShellChoice::Bash => ShellChoice::Fish,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationProfile {
    Snappy,
    Smooth,
    Minimal,
}

impl AnimationProfile {
    pub fn label(&self) -> &'static str {
        match self {
            AnimationProfile::Snappy => "Snappy (0.15s - Instant Response)",
            AnimationProfile::Smooth => "Smooth (0.3s - Elegant Curves)",
            AnimationProfile::Minimal => "Minimal (Reduced Motion)",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            AnimationProfile::Snappy => AnimationProfile::Smooth,
            AnimationProfile::Smooth => AnimationProfile::Minimal,
            AnimationProfile::Minimal => AnimationProfile::Snappy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserChoice {
    Zen,
    Brave,
    Firefox,
    Chromium,
    None,
}

#[allow(dead_code)]
impl BrowserChoice {
    pub const ALL: &[BrowserChoice] = &[
        BrowserChoice::Zen,
        BrowserChoice::Brave,
        BrowserChoice::Firefox,
        BrowserChoice::Chromium,
        BrowserChoice::None,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            BrowserChoice::Zen => "Zen Browser (Aurora Signature - Split Tabs & Privacy)",
            BrowserChoice::Brave => "Brave (Native Adblock & Privacy Shield)",
            BrowserChoice::Firefox => "Firefox (Official Wayland Open-Source)",
            BrowserChoice::Chromium => "Chromium (Pure Engine & High Compatibility)",
            BrowserChoice::None => "None / Keep Current Browser Untouched",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            BrowserChoice::Zen => "Zen Browser",
            BrowserChoice::Brave => "Brave",
            BrowserChoice::Firefox => "Firefox",
            BrowserChoice::Chromium => "Chromium",
            BrowserChoice::None => "None",
        }
    }

    pub fn package_name(&self) -> Option<&'static str> {
        match self {
            BrowserChoice::Zen => Some("zen-browser-bin"),
            BrowserChoice::Brave => Some("brave-bin"),
            BrowserChoice::Firefox => Some("firefox"),
            BrowserChoice::Chromium => Some("chromium"),
            BrowserChoice::None => None,
        }
    }

    pub fn is_aur(&self) -> bool {
        matches!(self, BrowserChoice::Zen | BrowserChoice::Brave)
    }

    pub fn cycle(&self) -> Self {
        match self {
            BrowserChoice::Zen => BrowserChoice::Brave,
            BrowserChoice::Brave => BrowserChoice::Firefox,
            BrowserChoice::Firefox => BrowserChoice::Chromium,
            BrowserChoice::Chromium => BrowserChoice::None,
            BrowserChoice::None => BrowserChoice::Zen,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayManagerChoice {
    SddmAstronaut,
    LyMatrix,
    GreetdTuigreet,
    NoneTty,
}

#[allow(dead_code)]
impl DisplayManagerChoice {
    pub const ALL: &[DisplayManagerChoice] = &[
        DisplayManagerChoice::SddmAstronaut,
        DisplayManagerChoice::LyMatrix,
        DisplayManagerChoice::GreetdTuigreet,
        DisplayManagerChoice::NoneTty,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            DisplayManagerChoice::SddmAstronaut => "SDDM (Astronaut Animated Theme)",
            DisplayManagerChoice::LyMatrix => "Ly (Ultra-Minimalist TUI Matrix)",
            DisplayManagerChoice::GreetdTuigreet => "Greetd (Tuigreet Wayland Greeter)",
            DisplayManagerChoice::NoneTty => "Manual TTY Login (Direct start-hyprland)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            DisplayManagerChoice::SddmAstronaut => "SDDM Astronaut",
            DisplayManagerChoice::LyMatrix => "Ly TUI",
            DisplayManagerChoice::GreetdTuigreet => "Tuigreet",
            DisplayManagerChoice::NoneTty => "TTY / Manual",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            DisplayManagerChoice::SddmAstronaut => DisplayManagerChoice::LyMatrix,
            DisplayManagerChoice::LyMatrix => DisplayManagerChoice::GreetdTuigreet,
            DisplayManagerChoice::GreetdTuigreet => DisplayManagerChoice::NoneTty,
            DisplayManagerChoice::NoneTty => DisplayManagerChoice::SddmAstronaut,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarLayoutChoice {
    FloatingIsland,
    FullWidthEdge,
    DualBar,
    MinimalClean,
}

#[allow(dead_code)]
impl BarLayoutChoice {
    pub const ALL: &[BarLayoutChoice] = &[
        BarLayoutChoice::FloatingIsland,
        BarLayoutChoice::FullWidthEdge,
        BarLayoutChoice::DualBar,
        BarLayoutChoice::MinimalClean,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            BarLayoutChoice::FloatingIsland => "Floating Islands (Rounded Pills & Gaps - Signature)",
            BarLayoutChoice::FullWidthEdge => "Full Width Edge (Solid Powerline Top Bar)",
            BarLayoutChoice::DualBar => "Dual Waybar (Top Status + Bottom Workspace Dock)",
            BarLayoutChoice::MinimalClean => "Minimal Clean (Uncluttered Time & Battery)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            BarLayoutChoice::FloatingIsland => "Floating Islands",
            BarLayoutChoice::FullWidthEdge => "Full Width Edge",
            BarLayoutChoice::DualBar => "Dual Waybar",
            BarLayoutChoice::MinimalClean => "Minimal Clean",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            BarLayoutChoice::FloatingIsland => BarLayoutChoice::FullWidthEdge,
            BarLayoutChoice::FullWidthEdge => BarLayoutChoice::DualBar,
            BarLayoutChoice::DualBar => BarLayoutChoice::MinimalClean,
            BarLayoutChoice::MinimalClean => BarLayoutChoice::FloatingIsland,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LauncherStyle {
    Spotlight,
    FullscreenDrawer,
    CornerPopup,
}

#[allow(dead_code)]
impl LauncherStyle {
    pub const ALL: &[LauncherStyle] = &[
        LauncherStyle::Spotlight,
        LauncherStyle::FullscreenDrawer,
        LauncherStyle::CornerPopup,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            LauncherStyle::Spotlight => "Spotlight (Center Floating Palette - Raycast Style)",
            LauncherStyle::FullscreenDrawer => "Fullscreen Drawer (GNOME/App Grid Style)",
            LauncherStyle::CornerPopup => "Corner Popup (Left-Aligned Desktop Launcher)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            LauncherStyle::Spotlight => "Center Spotlight",
            LauncherStyle::FullscreenDrawer => "Fullscreen Grid",
            LauncherStyle::CornerPopup => "Corner Popup",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            LauncherStyle::Spotlight => LauncherStyle::FullscreenDrawer,
            LauncherStyle::FullscreenDrawer => LauncherStyle::CornerPopup,
            LauncherStyle::CornerPopup => LauncherStyle::Spotlight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioSuiteChoice {
    EasyEffectsStudio,
    PurePipewire,
}

#[allow(dead_code)]
impl AudioSuiteChoice {
    pub const ALL: &[AudioSuiteChoice] = &[
        AudioSuiteChoice::EasyEffectsStudio,
        AudioSuiteChoice::PurePipewire,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            AudioSuiteChoice::EasyEffectsStudio => "EasyEffects Studio (DSP Equalizer & Bass Presets)",
            AudioSuiteChoice::PurePipewire => "Pure PipeWire + WirePlumber (Lightweight Audio)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            AudioSuiteChoice::EasyEffectsStudio => "EasyEffects Studio",
            AudioSuiteChoice::PurePipewire => "Pure PipeWire",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            AudioSuiteChoice::EasyEffectsStudio => AudioSuiteChoice::PurePipewire,
            AudioSuiteChoice::PurePipewire => AudioSuiteChoice::EasyEffectsStudio,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KernelTweakProfile {
    Performance,
    Standard,
}

#[allow(dead_code)]
impl KernelTweakProfile {
    pub const ALL: &[KernelTweakProfile] = &[
        KernelTweakProfile::Performance,
        KernelTweakProfile::Standard,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            KernelTweakProfile::Performance => "Ananicy-cpp + ZRAM (Max Gaming & UI Smoothness)",
            KernelTweakProfile::Standard => "Standard Arch Linux Kernel Scheduling",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            KernelTweakProfile::Performance => "Ananicy + ZRAM",
            KernelTweakProfile::Standard => "Standard Kernel",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            KernelTweakProfile::Performance => KernelTweakProfile::Standard,
            KernelTweakProfile::Standard => KernelTweakProfile::Performance,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontChoice {
    JetBrainsMono,
    FiraCode,
    CascadiaCode,
    Hack,
}

#[allow(dead_code)]
impl FontChoice {
    pub const ALL: &[FontChoice] = &[
        FontChoice::JetBrainsMono,
        FontChoice::FiraCode,
        FontChoice::CascadiaCode,
        FontChoice::Hack,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            FontChoice::JetBrainsMono => "JetBrains Mono Nerd Font (Aurora Signature)",
            FontChoice::FiraCode => "FiraCode Nerd Font (Programming Ligatures)",
            FontChoice::CascadiaCode => "Cascadia Code Nerd Font (Microsoft Clean)",
            FontChoice::Hack => "Hack Nerd Font (Classic Open Source)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            FontChoice::JetBrainsMono => "JetBrains Mono",
            FontChoice::FiraCode => "FiraCode",
            FontChoice::CascadiaCode => "Cascadia Code",
            FontChoice::Hack => "Hack",
        }
    }

    pub fn package_name(&self) -> &'static str {
        match self {
            FontChoice::JetBrainsMono => "ttf-jetbrains-mono-nerd",
            FontChoice::FiraCode => "ttf-firacode-nerd",
            FontChoice::CascadiaCode => "ttf-cascadia-code-nerd",
            FontChoice::Hack => "ttf-hack-nerd",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            FontChoice::JetBrainsMono => FontChoice::FiraCode,
            FontChoice::FiraCode => FontChoice::CascadiaCode,
            FontChoice::CascadiaCode => FontChoice::Hack,
            FontChoice::Hack => FontChoice::JetBrainsMono,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockscreenChoice {
    Hyprlock,
    SwaylockEffects,
    None,
}

#[allow(dead_code)]
impl LockscreenChoice {
    pub const ALL: &[LockscreenChoice] = &[
        LockscreenChoice::Hyprlock,
        LockscreenChoice::SwaylockEffects,
        LockscreenChoice::None,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            LockscreenChoice::Hyprlock => "Hyprlock (Aurora Animated + Blurred Wallpaper)",
            LockscreenChoice::SwaylockEffects => "Swaylock-effects (Fade & Blur Screen)",
            LockscreenChoice::None => "None / Disable Auto-Lock Daemon",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            LockscreenChoice::Hyprlock => "Hyprlock",
            LockscreenChoice::SwaylockEffects => "Swaylock-effects",
            LockscreenChoice::None => "None",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            LockscreenChoice::Hyprlock => LockscreenChoice::SwaylockEffects,
            LockscreenChoice::SwaylockEffects => LockscreenChoice::None,
            LockscreenChoice::None => LockscreenChoice::Hyprlock,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuProfile {
    AutoDetect,
    NvidiaProprietary,
    AmdMesa,
    IntelMesa,
    VirtualMachine,
}

#[allow(dead_code)]
impl GpuProfile {
    pub const ALL: &[GpuProfile] = &[
        GpuProfile::AutoDetect,
        GpuProfile::NvidiaProprietary,
        GpuProfile::AmdMesa,
        GpuProfile::IntelMesa,
        GpuProfile::VirtualMachine,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::AutoDetect => "Auto-Detect Hardware (Smart GPU Config)",
            Self::NvidiaProprietary => "NVIDIA Proprietary + Hyprland Envs (DKMS + Wayland)",
            Self::AmdMesa => "AMD Radeon Open-Source (Mesa + Vulkan-Radeon)",
            Self::IntelMesa => "Intel Iris / Arc Open-Source (Mesa + Vulkan-Intel)",
            Self::VirtualMachine => "Virtual Machine (VirtIO-GPU / VirGL 3D)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::AutoDetect => "Auto-Detect",
            Self::NvidiaProprietary => "NVIDIA DKMS",
            Self::AmdMesa => "AMD Mesa",
            Self::IntelMesa => "Intel Mesa",
            Self::VirtualMachine => "Virtual Machine",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::AutoDetect => Self::NvidiaProprietary,
            Self::NvidiaProprietary => Self::AmdMesa,
            Self::AmdMesa => Self::IntelMesa,
            Self::IntelMesa => Self::VirtualMachine,
            Self::VirtualMachine => Self::AutoDetect,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WallpaperEngine {
    Swww,
    Hyprpaper,
    Awww,
    MpvPaper,
}

#[allow(dead_code)]
impl WallpaperEngine {
    pub const ALL: &[WallpaperEngine] = &[
        WallpaperEngine::Swww,
        WallpaperEngine::Hyprpaper,
        WallpaperEngine::Awww,
        WallpaperEngine::MpvPaper,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Swww => "SWWW (Smooth Fluid Animated Transitions & GIFs)",
            Self::Hyprpaper => "Hyprpaper (Ultra-lightweight Fast C++ Daemon)",
            Self::Awww => "AWWW (Simple Wayland Wallpaper Daemon)",
            Self::MpvPaper => "mpvpaper (Live Desktop Video & Shaders)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Swww => "SWWW Transitions",
            Self::Hyprpaper => "Hyprpaper Native",
            Self::Awww => "AWWW Lightweight",
            Self::MpvPaper => "mpvpaper Video",
        }
    }

    pub fn package_name(&self) -> &'static str {
        match self {
            Self::Swww => "swww",
            Self::Hyprpaper => "hyprpaper",
            Self::Awww => "awww",
            Self::MpvPaper => "mpvpaper",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::Swww => Self::Hyprpaper,
            Self::Hyprpaper => Self::Awww,
            Self::Awww => Self::MpvPaper,
            Self::MpvPaper => Self::Swww,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupStrategy {
    Timestamped,
    DirectOverwrite,
    SkipExisting,
}

#[allow(dead_code)]
impl BackupStrategy {
    pub const ALL: &[BackupStrategy] = &[
        BackupStrategy::Timestamped,
        BackupStrategy::DirectOverwrite,
        BackupStrategy::SkipExisting,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Timestamped => "Timestamped Backup (~/.config/aurora_backups/)",
            Self::DirectOverwrite => "Direct Overwrite (Keep existing files without backup)",
            Self::SkipExisting => "Preserve Existing (Skip conflicting config files)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Timestamped => "Timestamped Backup",
            Self::DirectOverwrite => "Direct Overwrite",
            Self::SkipExisting => "Preserve Existing",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::Timestamped => Self::DirectOverwrite,
            Self::DirectOverwrite => Self::SkipExisting,
            Self::SkipExisting => Self::Timestamped,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Component {
    SystemPackages,
    ExtraRepositories,
    SddmTheme,
    RustScripts,
    Waytrogen,
    LazyVim,
    Dotfiles,
    ShellConfig,
    SudoRs,
    GamingSuite,
}

impl Component {
    pub const ALL: &[Component] = &[
        Component::SystemPackages,
        Component::ExtraRepositories,
        Component::SddmTheme,
        Component::RustScripts,
        Component::Waytrogen,
        Component::LazyVim,
        Component::Dotfiles,
        Component::ShellConfig,
        Component::SudoRs,
        Component::GamingSuite,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Component::SystemPackages => "System & Desktop Packages (Hyprland, Waybar, Rofi, Audio)",
            Component::ExtraRepositories => "Package Repositories (AUR / COPR / Flakes & Zen Browser)",
            Component::SddmTheme => "SDDM Astronaut Theme & Font Setup",
            Component::RustScripts => "Aurora Hyprland Rust Scripts & Tools (aurora CLI)",
            Component::Waytrogen => "Waytrogen-Aurora Wallpaper Switcher",
            Component::LazyVim => "Neovim IDE Configuration Setup",
            Component::Dotfiles => "Aurora Dotfiles (~/.config, preserves User customization)",
            Component::ShellConfig => "Shell PATH & Default Shell Configuration",
            Component::SudoRs => "Switch sudo to sudo-rs (Memory-safe Rust implementation)",
            Component::GamingSuite => "Gaming & Performance Suite (Steam, GameMode, MangoHud)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Component::SystemPackages => "Installs Hyprland, Waybar, SwayNC, Rofi, Pipewire, Wireplumber, fonts, and terminal",
            Component::ExtraRepositories => "Enables AUR helpers, Fedora COPR repositories, or Nix Flakes for extra ricing tools",
            Component::SddmTheme => "Installs SDDM login manager astronaut theme with elegant animations and fonts",
            Component::RustScripts => "Compiles and installs Aurora's high-performance Rust helper scripts into ~/.cargo/bin",
            Component::Waytrogen => "Integrates the fast GTK/Rust wallpaper daemon and management tool for Wayland",
            Component::LazyVim => "Deploys selected Neovim IDE preset (LazyVim, NvChad, AstroNvim, Kickstart)",
            Component::Dotfiles => "Safely deploys Waybar, Hyprland, Kitty, Ghostty, and SwayNC configs with timestamped backups",
            Component::ShellConfig => "Configures environment variables, default shell, and adds ~/.cargo/bin and ~/.local/bin to PATH",
            Component::SudoRs => "Optionally symlinks /usr/bin/sudo to /usr/bin/sudo-rs for modern Rust memory safety",
            Component::GamingSuite => "Installs Steam, GameMode optimizer daemon, MangoHud FPS overlay, and gaming tweaks",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallConfig {
    pub mode: InstallMode,
    pub preset: InstallPreset,
    pub selected_components: Vec<Component>,
    pub neovim_preset: NeovimPreset,
    pub terminal_choice: TerminalChoice,
    pub browser_choice: BrowserChoice,
    pub display_manager: DisplayManagerChoice,
    pub bar_layout: BarLayoutChoice,
    pub launcher_style: LauncherStyle,
    pub font_choice: FontChoice,
    pub lockscreen: LockscreenChoice,
    pub gpu_profile: GpuProfile,
    pub wallpaper_engine: WallpaperEngine,
    pub backup_strategy: BackupStrategy,
    pub audio_suite: AudioSuiteChoice,
    pub kernel_tweaks: KernelTweakProfile,
    pub shell_choice: ShellChoice,
    pub animation_profile: AnimationProfile,
    pub theme_name: String,
    pub dry_run: bool,
    pub backup_dir: PathBuf,
    pub source_dir: PathBuf,
    pub allow_sudo_rs: bool,
    pub disabled_packages: HashSet<String>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let home = dirs_home().unwrap_or_else(|| PathBuf::from("/tmp"));
        let backup_dir = home.join(format!(".config/aurora_backup_{}", timestamp));

        // Default components for FullRice (GamingSuite optional by default)
        let selected_components = vec![
            Component::SystemPackages,
            Component::ExtraRepositories,
            Component::SddmTheme,
            Component::RustScripts,
            Component::Waytrogen,
            Component::LazyVim,
            Component::Dotfiles,
            Component::ShellConfig,
            Component::SudoRs,
        ];

        Self {
            mode: InstallMode::Stable,
            preset: InstallPreset::FullRice,
            selected_components,
            neovim_preset: NeovimPreset::LazyVim,
            terminal_choice: TerminalChoice::Kitty,
            browser_choice: BrowserChoice::Zen,
            display_manager: DisplayManagerChoice::SddmAstronaut,
            bar_layout: BarLayoutChoice::FloatingIsland,
            launcher_style: LauncherStyle::Spotlight,
            font_choice: FontChoice::JetBrainsMono,
            lockscreen: LockscreenChoice::Hyprlock,
            gpu_profile: GpuProfile::AutoDetect,
            wallpaper_engine: WallpaperEngine::Swww,
            backup_strategy: BackupStrategy::Timestamped,
            audio_suite: AudioSuiteChoice::EasyEffectsStudio,
            kernel_tweaks: KernelTweakProfile::Performance,
            shell_choice: ShellChoice::Fish,
            animation_profile: AnimationProfile::Snappy,
            theme_name: "Aurora Default".to_string(),
            dry_run: false,
            backup_dir,
            source_dir: PathBuf::new(),
            allow_sudo_rs: true,
            disabled_packages: HashSet::new(),
        }
    }
}

impl InstallConfig {
    pub fn is_component_selected(&self, component: Component) -> bool {
        self.selected_components.contains(&component)
    }

    pub fn toggle_component(&mut self, component: Component) {
        if let Some(pos) = self.selected_components.iter().position(|c| *c == component) {
            self.selected_components.remove(pos);
        } else {
            self.selected_components.push(component);
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn apply_preset(&mut self, preset: InstallPreset) {
        self.preset = preset;
        match preset {
            InstallPreset::FullRice => {
                self.selected_components = vec![
                    Component::SystemPackages,
                    Component::ExtraRepositories,
                    Component::SddmTheme,
                    Component::RustScripts,
                    Component::Waytrogen,
                    Component::LazyVim,
                    Component::Dotfiles,
                    Component::ShellConfig,
                    Component::SudoRs,
                ];
                self.disabled_packages.clear();
            }
            InstallPreset::MinimalRice => {
                self.selected_components = vec![
                    Component::SystemPackages,
                    Component::Dotfiles,
                    Component::RustScripts,
                    Component::ShellConfig,
                ];
                self.neovim_preset = NeovimPreset::None;
                self.browser_choice = BrowserChoice::None;
                self.display_manager = DisplayManagerChoice::NoneTty;
                self.disabled_packages.clear();
            }
            InstallPreset::Custom => {}
        }
    }

    pub fn is_package_enabled(&self, package_name: &str) -> bool {
        !self.disabled_packages.contains(package_name)
    }

    pub fn enable_package(&mut self, package_name: &str) {
        self.disabled_packages.remove(package_name);
        self.preset = InstallPreset::Custom;
    }

    pub fn disable_package(&mut self, package_name: &str) {
        self.disabled_packages.insert(package_name.to_string());
        self.preset = InstallPreset::Custom;
    }

    pub fn toggle_package(&mut self, package_name: &str) {
        if self.disabled_packages.contains(package_name) {
            self.disabled_packages.remove(package_name);
        } else {
            self.disabled_packages.insert(package_name.to_string());
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn enable_all_packages<'a>(&mut self, pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in pkgs {
            self.disabled_packages.remove(pkg);
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn disable_all_packages<'a>(&mut self, pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in pkgs {
            self.disabled_packages.insert(pkg.to_string());
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn enable_all_global(&mut self) {
        self.disabled_packages.clear();
        self.preset = InstallPreset::Custom;
    }

    pub fn disable_all_global<'a>(&mut self, all_pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in all_pkgs {
            self.disabled_packages.insert(pkg.to_string());
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn invert_all_global<'a>(&mut self, all_pkgs: impl IntoIterator<Item = &'a str>) {
        for pkg in all_pkgs {
            if self.disabled_packages.contains(pkg) {
                self.disabled_packages.remove(pkg);
            } else {
                self.disabled_packages.insert(pkg.to_string());
            }
        }
        self.preset = InstallPreset::Custom;
    }

    pub fn export_to_json(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    pub fn import_from_json(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

pub fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_profile_cycling() {
        let g = GpuProfile::AutoDetect;
        assert_eq!(g.cycle(), GpuProfile::NvidiaProprietary);
        assert_eq!(g.cycle().cycle(), GpuProfile::AmdMesa);
        assert_eq!(g.cycle().cycle().cycle(), GpuProfile::IntelMesa);
        assert_eq!(g.cycle().cycle().cycle().cycle(), GpuProfile::VirtualMachine);
        assert_eq!(g.cycle().cycle().cycle().cycle().cycle(), GpuProfile::AutoDetect);
    }

    #[test]
    fn test_wallpaper_engine_cycling() {
        let w = WallpaperEngine::Swww;
        assert_eq!(w.cycle(), WallpaperEngine::Hyprpaper);
        assert_eq!(w.cycle().cycle(), WallpaperEngine::Awww);
        assert_eq!(w.cycle().cycle().cycle(), WallpaperEngine::MpvPaper);
        assert_eq!(w.cycle().cycle().cycle().cycle(), WallpaperEngine::Swww);
    }

    #[test]
    fn test_backup_strategy_cycling() {
        let b = BackupStrategy::Timestamped;
        assert_eq!(b.cycle(), BackupStrategy::DirectOverwrite);
        assert_eq!(b.cycle().cycle(), BackupStrategy::SkipExisting);
        assert_eq!(b.cycle().cycle().cycle(), BackupStrategy::Timestamped);
    }

    #[test]
    fn test_profile_json_roundtrip() {
        let config = InstallConfig::default();
        let json = serde_json::to_string_pretty(&config).expect("Serialize to JSON");
        let restored: InstallConfig = serde_json::from_str(&json).expect("Deserialize from JSON");
        assert_eq!(config.theme_name, restored.theme_name);
        assert_eq!(config.gpu_profile, restored.gpu_profile);
        assert_eq!(config.wallpaper_engine, restored.wallpaper_engine);
        assert_eq!(config.backup_strategy, restored.backup_strategy);
    }

    #[test]
    fn test_font_choice_cycling() {
        let f = FontChoice::JetBrainsMono;
        assert_eq!(f.cycle(), FontChoice::FiraCode);
        assert_eq!(f.cycle().cycle(), FontChoice::CascadiaCode);
        assert_eq!(f.cycle().cycle().cycle(), FontChoice::Hack);
        assert_eq!(f.cycle().cycle().cycle().cycle(), FontChoice::JetBrainsMono);
    }

    #[test]
    fn test_lockscreen_choice_cycling() {
        let l = LockscreenChoice::Hyprlock;
        assert_eq!(l.cycle(), LockscreenChoice::SwaylockEffects);
        assert_eq!(l.cycle().cycle(), LockscreenChoice::None);
        assert_eq!(l.cycle().cycle().cycle(), LockscreenChoice::Hyprlock);
    }

    #[test]
    fn test_package_selection_certain_packages() {
        let mut cfg = InstallConfig::default();
        let all = ["hyprland", "waybar", "kitty", "rofi"];

        // Start with clean slate: deselect all
        cfg.disable_all_global(all);
        assert!(!cfg.is_package_enabled("hyprland"));
        assert!(!cfg.is_package_enabled("waybar"));

        // Select only certain packages
        cfg.enable_package("hyprland");
        cfg.enable_package("kitty");

        assert!(cfg.is_package_enabled("hyprland"));
        assert!(cfg.is_package_enabled("kitty"));
        assert!(!cfg.is_package_enabled("waybar"));
        assert!(!cfg.is_package_enabled("rofi"));
        assert_eq!(cfg.preset, InstallPreset::Custom);

        // Toggle
        cfg.toggle_package("waybar");
        assert!(cfg.is_package_enabled("waybar"));
    }
}
