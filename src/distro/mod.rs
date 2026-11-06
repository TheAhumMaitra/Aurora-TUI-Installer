// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod arch;
pub mod fedora;
pub mod nixos;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::{Disks, System};
use which::which;

#[derive(Debug, Clone, Copy)]
pub struct DistroPackageGroup {
    pub category: &'static str,
    pub description: &'static str,
    pub packages: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistroFamily {
    Arch,
    Fedora,
    NixOS,
    Unknown,
}

impl DistroFamily {
    pub fn name(&self) -> &'static str {
        match self {
            DistroFamily::Arch => "Arch Linux",
            DistroFamily::Fedora => "Fedora Linux",
            DistroFamily::NixOS => "NixOS",
            DistroFamily::Unknown => "Unsupported Linux",
        }
    }

    pub fn package_manager(&self) -> &'static str {
        match self {
            DistroFamily::Arch => "pacman / yay",
            DistroFamily::Fedora => "dnf / copr",
            DistroFamily::NixOS => "nix / flakes",
            DistroFamily::Unknown => "unknown",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            DistroFamily::Arch => "󰣇",
            DistroFamily::Fedora => "",
            DistroFamily::NixOS => "",
            DistroFamily::Unknown => "󰌽",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistroInfo {
    pub family: DistroFamily,
    pub id: String,
    pub pretty_name: String,
    pub id_like: Vec<String>,
    pub kernel: String,
    pub arch: String,
    pub current_user: String,
    pub is_root: bool,
    pub home_free_gb: f64,
    pub total_ram_gb: f64,
    pub available_ram_gb: f64,
    pub cpu_cores: usize,
    pub git_available: bool,
    pub cargo_available: bool,
    pub hyprland_running: bool,
}

impl DistroInfo {
    pub fn detect() -> Self {
        let (id, pretty_name, id_like) = read_os_release();

        let family = if Path::new("/etc/NIXOS").exists()
            || id == "nixos"
            || which("nixos-rebuild").is_ok()
        {
            DistroFamily::NixOS
        } else if is_arch_family(&id, &id_like) || which("pacman").is_ok() {
            DistroFamily::Arch
        } else if is_fedora_family(&id, &id_like) || which("dnf").is_ok() {
            DistroFamily::Fedora
        } else {
            DistroFamily::Unknown
        };

        // System specs via sysinfo
        let mut sys = System::new_all().or_else(|_| System::new()).expect("Failed to initialize sysinfo");
        sys.refresh_all();

        let kernel = System::kernel_version()
            .ok()
            .unwrap_or_else(|| "Linux".to_string());
        let arch = std::env::consts::ARCH.to_string();

        let current_user = std::env::var("USER")
            .unwrap_or_else(|_| std::env::var("LOGNAME").unwrap_or_else(|_| "user".to_string()));

        let is_root = is_current_user_root();

        // Calculate free disk space in HOME
        let home_dir = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/"));
        let mut home_free_gb = 50.0;
        if let Ok(disks) = Disks::new_with_refreshed_list() {
            for disk in &disks {
                if home_dir.starts_with(disk.mount_point()) {
                    home_free_gb = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                    break;
                }
            }
            if home_free_gb == 50.0 {
                if let Some(first) = disks.first() {
                    home_free_gb = first.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                }
            }
        }

        let total_ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_ram_gb = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let cpu_cores = sys.cpus().len();

        let git_available = which("git").is_ok();
        let cargo_available = which("cargo").is_ok();
        let hyprland_running = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some()
            || std::env::var("XDG_CURRENT_DESKTOP")
                .map(|v| v.to_lowercase().contains("hyprland"))
                .unwrap_or(false);

        Self {
            family,
            id,
            pretty_name,
            id_like,
            kernel,
            arch,
            current_user,
            is_root,
            home_free_gb,
            total_ram_gb,
            available_ram_gb,
            cpu_cores,
            git_available,
            cargo_available,
            hyprland_running,
        }
    }
}

fn read_os_release() -> (String, String, Vec<String>) {
    let mut id = "unknown".to_string();
    let mut pretty_name = "Linux".to_string();
    let mut id_like = Vec::new();

    let paths = ["/etc/os-release", "/usr/lib/os-release"];
    for p in paths {
        if let Ok(content) = fs::read_to_string(p) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((k, v)) = line.split_once('=') {
                    let k = k.trim();
                    let v = v.trim().trim_matches('"').trim_matches('\'').to_string();
                    match k {
                        "ID" => id = v.to_lowercase(),
                        "PRETTY_NAME" => pretty_name = v,
                        "ID_LIKE" => {
                            id_like = v
                                .split_whitespace()
                                .map(|s| s.to_lowercase())
                                .collect();
                        }
                        _ => {}
                    }
                }
            }
            break;
        }
    }

    (id, pretty_name, id_like)
}

fn is_arch_family(id: &str, id_like: &[String]) -> bool {
    let arch_ids = [
        "arch",
        "archarm",
        "archlinux",
        "cachyos",
        "endeavouros",
        "manjaro",
        "garuda",
        "artix",
        "arcolinux",
        "archcraft",
    ];

    if arch_ids.iter().any(|&candidate| candidate == id) {
        return true;
    }

    id_like.iter().any(|like| like == "arch")
}

fn is_fedora_family(id: &str, id_like: &[String]) -> bool {
    let fedora_ids = ["fedora", "nobara", "bazzite", "ultramarine", "rhel", "centos"];

    if fedora_ids.iter().any(|&candidate| candidate == id) {
        return true;
    }

    id_like
        .iter()
        .any(|like| like == "fedora" || like == "rhel" || like == "centos")
}

fn is_current_user_root() -> bool {
    // Check EUID on Unix
    if let Ok(euid) = std::env::var("EUID") {
        if euid == "0" {
            return true;
        }
    }
    // Alternatively check via whoami
    if let Ok(user) = std::env::var("USER") {
        if user == "root" {
            return true;
        }
    }
    false
}
