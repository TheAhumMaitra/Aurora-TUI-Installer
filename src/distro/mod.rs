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
            DistroFamily::Arch => "Arch Linux & Arch-based",
            DistroFamily::Fedora => "Fedora Linux",
            DistroFamily::NixOS => "NixOS",
            DistroFamily::Unknown => "Unsupported Linux",
        }
    }

    pub fn package_manager(&self) -> &'static str {
        match self {
            DistroFamily::Arch => "pacman / yay / paru",
            DistroFamily::Fedora => "dnf / copr",
            DistroFamily::NixOS => "nix / flakes",
            DistroFamily::Unknown => "unknown",
        }
    }

    #[allow(dead_code)]
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
    pub is_arch_compatible: bool,
    pub arch_flavor: String,
    pub aur_helper: Option<String>,
    pub pacman_available: bool,
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

        let is_arch = is_arch_family(&id, &id_like);
        let pacman_available = which("pacman").is_ok();
        let is_arch_compatible = is_arch || pacman_available;

        let family = if is_arch_compatible {
            DistroFamily::Arch
        } else if id == "fedora" || is_fedora_family(&id, &id_like) {
            DistroFamily::Fedora
        } else if id == "nixos" || Path::new("/etc/NIXOS").exists() {
            DistroFamily::NixOS
        } else {
            DistroFamily::Unknown
        };

        let arch_flavor = detect_arch_flavor(&id, &pretty_name, is_arch_compatible);

        let aur_helper = if which("paru").is_ok() {
            Some("paru".to_string())
        } else if which("yay").is_ok() {
            Some("yay".to_string())
        } else {
            None
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
            is_arch_compatible,
            arch_flavor,
            aur_helper,
            pacman_available,
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

pub fn detect_arch_flavor(id: &str, pretty_name: &str, is_arch_compat: bool) -> String {
    if !is_arch_compat {
        return pretty_name.to_string();
    }

    match id {
        "cachyos" => "CachyOS Linux".to_string(),
        "endeavouros" => "EndeavourOS".to_string(),
        "manjaro" | "manjaro-arm" => "Manjaro Linux".to_string(),
        "garuda" => "Garuda Linux".to_string(),
        "arch" | "archlinux" => "Arch Linux (Pure)".to_string(),
        "archarm" => "Arch Linux ARM".to_string(),
        "artix" => "Artix Linux".to_string(),
        "arcolinux" => "ArcoLinux".to_string(),
        "archcraft" => "Archcraft".to_string(),
        "blackarch" => "BlackArch Linux".to_string(),
        "parabola" => "Parabola GNU/Linux-libre".to_string(),
        "hyperbola" => "Hyperbola GNU/Linux-libre".to_string(),
        "steamos" => "SteamOS (Arch derivative)".to_string(),
        _ => {
            if pretty_name.to_lowercase().contains("arch") {
                pretty_name.to_string()
            } else {
                format!("{pretty_name} (Arch-compatible)")
            }
        }
    }
}

pub fn is_arch_id(id: &str, id_like: &[String]) -> bool {
    let arch_ids = [
        "arch",
        "archarm",
        "archlinux",
        "cachyos",
        "endeavouros",
        "manjaro",
        "manjaro-arm",
        "garuda",
        "artix",
        "arcolinux",
        "archcraft",
        "blackarch",
        "parabola",
        "hyperbola",
        "salient",
        "steamos",
    ];

    if arch_ids.iter().any(|&candidate| candidate == id) {
        return true;
    }

    id_like.iter().any(|like| like == "arch")
}

pub fn is_arch_family(id: &str, id_like: &[String]) -> bool {
    is_arch_id(id, id_like) || Path::new("/etc/arch-release").exists() || which("pacman").is_ok()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_family_detection() {
        assert!(is_arch_id("arch", &[]));
        assert!(is_arch_id("cachyos", &["arch".to_string()]));
        assert!(is_arch_id("endeavouros", &["arch".to_string()]));
        assert!(is_arch_id("manjaro", &[]));
        assert!(is_arch_id("garuda", &[]));
        assert!(is_arch_id("artix", &[]));
        assert!(is_arch_id("arcolinux", &[]));
        assert!(is_arch_id("archcraft", &[]));
        assert!(is_arch_id("steamos", &[]));

        let ubuntu_like = vec!["debian".to_string()];
        assert!(!is_arch_id("ubuntu", &ubuntu_like));
        assert!(!is_arch_id("fedora", &[]));
    }

    #[test]
    fn test_arch_flavor_names() {
        assert_eq!(detect_arch_flavor("cachyos", "CachyOS Linux", true), "CachyOS Linux");
        assert_eq!(detect_arch_flavor("arch", "Arch Linux", true), "Arch Linux (Pure)");
        assert_eq!(detect_arch_flavor("endeavouros", "EndeavourOS", true), "EndeavourOS");
        assert_eq!(detect_arch_flavor("garuda", "Garuda Linux", true), "Garuda Linux");
    }
}
