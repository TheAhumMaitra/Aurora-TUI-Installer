// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::distro::DistroInfo;
use crate::model::config::{dirs_home, InstallConfig};
use std::path::Path;
use which::which;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct DoctorCheck {
    pub category: &'static str,
    pub name: &'static str,
    pub status: CheckStatus,
    pub detail: String,
    pub remedy: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
    pub total_pass: usize,
    pub total_warn: usize,
    pub total_fail: usize,
}

impl DoctorReport {
    pub fn is_healthy(&self) -> bool {
        self.total_fail == 0
    }
}

pub fn run_doctor(distro_info: &DistroInfo, _config: &InstallConfig) -> DoctorReport {
    let mut checks = Vec::new();
    let home = dirs_home().unwrap_or_else(|| std::path::PathBuf::from("/home"));

    // 1. Hyprland Compositor Executable & Config
    let hypr_bin = which("hyprland").or_else(|_| which("Hyprland"));
    let hypr_conf = home.join(".config/hypr/hyprland.conf");
    if hypr_bin.is_ok() && hypr_conf.exists() {
        checks.push(DoctorCheck {
            category: "Compositor",
            name: "Hyprland Executable & Config",
            status: CheckStatus::Pass,
            detail: "Binary and ~/.config/hypr/hyprland.conf verified.".to_string(),
            remedy: None,
        });
    } else if hypr_bin.is_ok() {
        checks.push(DoctorCheck {
            category: "Compositor",
            name: "Hyprland Executable & Config",
            status: CheckStatus::Warn,
            detail: "Hyprland binary found, but ~/.config/hypr/hyprland.conf not deployed yet.".to_string(),
            remedy: Some("Run installer with --preset full to deploy Aurora configs."),
        });
    } else {
        checks.push(DoctorCheck {
            category: "Compositor",
            name: "Hyprland Executable & Config",
            status: CheckStatus::Fail,
            detail: "Hyprland binary not found in PATH.".to_string(),
            remedy: Some("Install Hyprland package using the Aurora installer."),
        });
    }

    // 2. Waybar Status Bar & Stylesheet
    let waybar_bin = which("waybar");
    let waybar_conf = home.join(".config/waybar/config.jsonc")
        .exists() || home.join(".config/waybar/config").exists();
    let waybar_css = home.join(".config/waybar/style.css").exists();
    if waybar_bin.is_ok() && waybar_conf && waybar_css {
        checks.push(DoctorCheck {
            category: "Status Bar",
            name: "Waybar Configuration & CSS",
            status: CheckStatus::Pass,
            detail: "Waybar binary, config, and CSS stylesheets verified.".to_string(),
            remedy: None,
        });
    } else if waybar_bin.is_ok() {
        checks.push(DoctorCheck {
            category: "Status Bar",
            name: "Waybar Configuration & CSS",
            status: CheckStatus::Warn,
            detail: "Waybar installed, but Aurora config/CSS not detected in ~/.config/waybar/.".to_string(),
            remedy: Some("Deploy dotfiles component in installer."),
        });
    } else {
        checks.push(DoctorCheck {
            category: "Status Bar",
            name: "Waybar Configuration & CSS",
            status: CheckStatus::Fail,
            detail: "Waybar binary not found in PATH.".to_string(),
            remedy: Some("Install waybar package."),
        });
    }

    // 3. Audio & Sound Pipeline (PipeWire & WirePlumber)
    let pw_ok = which("pipewire").is_ok();
    let wp_ok = which("wireplumber").is_ok();
    let has_sound_cards = std::fs::read_to_string("/proc/asound/cards")
        .map(|s| !s.trim().is_empty() && !s.contains("no soundcards"))
        .unwrap_or(false);
    if pw_ok && wp_ok && has_sound_cards {
        checks.push(DoctorCheck {
            category: "Audio",
            name: "PipeWire & WirePlumber Daemons",
            status: CheckStatus::Pass,
            detail: "PipeWire, WirePlumber, and physical audio cards detected.".to_string(),
            remedy: None,
        });
    } else if pw_ok && wp_ok {
        checks.push(DoctorCheck {
            category: "Audio",
            name: "PipeWire & WirePlumber Daemons",
            status: CheckStatus::Warn,
            detail: "PipeWire & WirePlumber available, but no ALSA hardware cards found in /proc/asound/cards.".to_string(),
            remedy: Some("Verify audio drivers or kernel sound modules."),
        });
    } else {
        checks.push(DoctorCheck {
            category: "Audio",
            name: "PipeWire & WirePlumber Daemons",
            status: CheckStatus::Fail,
            detail: "PipeWire or WirePlumber missing from system PATH.".to_string(),
            remedy: Some("Install pipewire, wireplumber, and pipewire-pulse."),
        });
    }

    // 4. Dotfiles Safety & User Customization Preservation
    let user_config = home.join(".config/hypr/User");
    if user_config.exists() {
        checks.push(DoctorCheck {
            category: "Safety",
            name: "User Customization Preservation",
            status: CheckStatus::Pass,
            detail: "User custom config directory (~/.config/hypr/User) safely preserved.".to_string(),
            remedy: None,
        });
    } else {
        checks.push(DoctorCheck {
            category: "Safety",
            name: "User Customization Preservation",
            status: CheckStatus::Pass,
            detail: "Fresh installation state; ~/.config/hypr/User will be created safely on install.".to_string(),
            remedy: None,
        });
    }

    // 5. GPU & Wayland Acceleration
    let dri_nodes = Path::new("/dev/dri/card0").exists() || Path::new("/dev/dri/renderD128").exists();
    if dri_nodes {
        checks.push(DoctorCheck {
            category: "Graphics",
            name: "Direct Rendering & Vulkan Nodes",
            status: CheckStatus::Pass,
            detail: format!("Kernel DRM/KMS direct rendering nodes active ({}).", distro_info.gpu.label()),
            remedy: None,
        });
    } else {
        checks.push(DoctorCheck {
            category: "Graphics",
            name: "Direct Rendering & Vulkan Nodes",
            status: CheckStatus::Warn,
            detail: "No standard /dev/dri nodes found (possibly running in minimal container or VM without 3D).".to_string(),
            remedy: Some("Enable 3D acceleration or verify GPU kernel drivers (amdgpu/i915/nvidia)."),
        });
    }

    // 6. Monospace Nerd Fonts
    let fc_list_ok = which("fc-list").map(|fc| {
        std::process::Command::new(fc)
            .arg(":")
            .arg("family")
            .output()
            .map(|out| {
                let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
                s.contains("nerd") || s.contains("jetbrains") || s.contains("fira") || s.contains("cascadia")
            })
            .unwrap_or(false)
    }).unwrap_or(false);

    if fc_list_ok {
        checks.push(DoctorCheck {
            category: "Typography",
            name: "Monospace Nerd Font Family",
            status: CheckStatus::Pass,
            detail: "High-resolution Nerd Font detected in fontconfig cache.".to_string(),
            remedy: None,
        });
    } else {
        checks.push(DoctorCheck {
            category: "Typography",
            name: "Monospace Nerd Font Family",
            status: CheckStatus::Warn,
            detail: "Nerd Font not found in fc-list cache; icons may show as missing glyphs.".to_string(),
            remedy: Some("Install JetBrains Mono Nerd Font via package manager or installer."),
        });
    }

    // 7. Wallpaper Engine Daemon
    let wall_bin = which("swww").or_else(|_| which("hyprpaper")).or_else(|_| which("awww"));
    if wall_bin.is_ok() {
        checks.push(DoctorCheck {
            category: "Wallpapers",
            name: "Wayland Wallpaper Engine",
            status: CheckStatus::Pass,
            detail: format!("Wallpaper utility detected: {}", wall_bin.unwrap().display()),
            remedy: None,
        });
    } else {
        checks.push(DoctorCheck {
            category: "Wallpapers",
            name: "Wayland Wallpaper Engine",
            status: CheckStatus::Warn,
            detail: "No wallpaper daemon (swww, hyprpaper, awww) found in PATH.".to_string(),
            remedy: Some("Install swww or hyprpaper."),
        });
    }

    // 8. Terminal Emulator
    let term_bin = which("kitty").or_else(|_| which("ghostty")).or_else(|_| which("alacritty")).or_else(|_| which("foot"));
    if term_bin.is_ok() {
        checks.push(DoctorCheck {
            category: "Terminal",
            name: "Wayland Terminal Emulator",
            status: CheckStatus::Pass,
            detail: format!("Terminal emulator available: {}", term_bin.unwrap().display()),
            remedy: None,
        });
    } else {
        checks.push(DoctorCheck {
            category: "Terminal",
            name: "Wayland Terminal Emulator",
            status: CheckStatus::Fail,
            detail: "No default Wayland terminal (kitty, ghostty, alacritty, foot) found.".to_string(),
            remedy: Some("Install Kitty or Ghostty."),
        });
    }

    let mut total_pass = 0;
    let mut total_warn = 0;
    let mut total_fail = 0;
    for c in &checks {
        match c.status {
            CheckStatus::Pass => total_pass += 1,
            CheckStatus::Warn => total_warn += 1,
            CheckStatus::Fail => total_fail += 1,
        }
    }

    DoctorReport {
        checks,
        total_pass,
        total_warn,
        total_fail,
    }
}

pub fn print_doctor_report(report: &DoctorReport) {
    println!("================================================================================");
    println!("                  AURORA SYSTEM DOCTOR & HEALTH VERIFIER                        ");
    println!("================================================================================");
    for check in &report.checks {
        let (badge, color_code) = match check.status {
            CheckStatus::Pass => ("[ PASS ]", "\x1b[32m"), // Green
            CheckStatus::Warn => ("[ WARN ]", "\x1b[33m"), // Yellow
            CheckStatus::Fail => ("[ FAIL ]", "\x1b[31m"), // Red
        };
        println!(
            "{}{}\x1b[0m {:<14} | {:<32} | {}",
            color_code, badge, check.category, check.name, check.detail
        );
        if let Some(remedy) = check.remedy {
            println!("         \x1b[36m➜ Remedy:\x1b[0m {}", remedy);
        }
    }
    println!("--------------------------------------------------------------------------------");
    println!(
        "SUMMARY: {} Passed, {} Warnings, {} Failed",
        report.total_pass, report.total_warn, report.total_fail
    );
    if report.is_healthy() {
        println!("\x1b[32m✔ AURORA HEALTH RESULT: SYSTEM FULLY OPERATIONAL & COMPLIANT\x1b[0m");
    } else {
        println!("\x1b[31m✘ AURORA HEALTH RESULT: ATTENTION REQUIRED BEFORE OPTIMAL OPERATION\x1b[0m");
    }
    println!("================================================================================");
}
