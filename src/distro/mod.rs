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
            DistroFamily::Fedora => "Fedora Linux & Fedora-based",
            DistroFamily::NixOS => "NixOS",
            DistroFamily::Unknown => "Unsupported Linux",
        }
    }

    pub fn label(&self) -> &'static str {
        self.name()
    }

    pub fn package_manager(&self) -> &'static str {
        match self {
            DistroFamily::Arch => "pacman / yay / paru",
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

    pub fn cycle(&self) -> Self {
        match self {
            DistroFamily::Arch => DistroFamily::Fedora,
            DistroFamily::Fedora => DistroFamily::NixOS,
            DistroFamily::NixOS => DistroFamily::Arch,
            DistroFamily::Unknown => DistroFamily::Arch,
        }
    }

    pub fn package_groups(&self) -> &'static [DistroPackageGroup] {
        match self {
            DistroFamily::Arch => arch::ARCH_PACKAGE_GROUPS,
            DistroFamily::Fedora => fedora::FEDORA_PACKAGE_GROUPS,
            DistroFamily::NixOS => nixos::NIXOS_PACKAGE_GROUPS,
            DistroFamily::Unknown => &[],
        }
    }

    pub fn all_packages(&self) -> Vec<&'static str> {
        match self {
            DistroFamily::Arch => arch::all_arch_packages(),
            DistroFamily::Fedora => fedora::all_fedora_packages(),
            DistroFamily::NixOS => nixos::all_nixos_packages(),
            DistroFamily::Unknown => Vec::new(),
        }
    }

    pub fn package_badge(&self, pkg: &str) -> &'static str {
        match self {
            DistroFamily::Arch => arch::package_category_badge(pkg),
            DistroFamily::Fedora => fedora::package_category_badge(pkg),
            DistroFamily::NixOS => nixos::package_category_badge(pkg),
            DistroFamily::Unknown => "Extra",
        }
    }

    pub fn build_steps(&self, config: &crate::model::config::InstallConfig) -> Vec<crate::engine::step::InstallStep> {
        match self {
            DistroFamily::Arch => arch::build_steps(config),
            DistroFamily::Fedora => fedora::build_steps(config),
            DistroFamily::NixOS => nixos::build_steps(config),
            DistroFamily::Unknown => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectedGpu {
    Nvidia(String),
    Amd(String),
    Intel(String),
    Hybrid { primary: String, discrete: String },
    VirtualMachine(String),
    Unknown,
}

impl DetectedGpu {
    pub fn label(&self) -> String {
        match self {
            Self::Nvidia(name) => format!("NVIDIA ({name})"),
            Self::Amd(name) => format!("AMD Radeon ({name})"),
            Self::Intel(name) => format!("Intel Iris/UHD ({name})"),
            Self::Hybrid { primary, discrete } => format!("Hybrid ({primary} + {discrete})"),
            Self::VirtualMachine(name) => format!("Virtual Machine ({name})"),
            Self::Unknown => "Generic / Unknown Display Adapter".to_string(),
        }
    }

    pub fn is_nvidia(&self) -> bool {
        matches!(self, Self::Nvidia(_) | Self::Hybrid { .. })
    }
}

pub fn is_uefi_mode() -> bool {
    Path::new("/sys/firmware/efi").is_dir()
}

pub fn detect_gpu() -> DetectedGpu {
    // 1. Try lspci for exact vendor and model strings
    if let Ok(output) = std::process::Command::new("lspci").arg("-nn").output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut vga_lines = Vec::new();
            for line in text.lines() {
                let lower = line.to_lowercase();
                if lower.contains("vga compatible controller")
                    || lower.contains("3d controller")
                    || lower.contains("display controller")
                {
                    vga_lines.push(line.to_string());
                }
            }

            if !vga_lines.is_empty() {
                let has_nvidia = vga_lines
                    .iter()
                    .any(|l| l.contains("10de:") || l.to_lowercase().contains("nvidia"));
                let has_amd = vga_lines.iter().any(|l| {
                    l.contains("1002:")
                        || l.to_lowercase().contains("amd")
                        || l.to_lowercase().contains("advanced micro devices")
                });
                let has_intel = vga_lines
                    .iter()
                    .any(|l| l.contains("8086:") || l.to_lowercase().contains("intel"));
                let has_vm = vga_lines.iter().any(|l| {
                    let l = l.to_lowercase();
                    l.contains("virtio")
                        || l.contains("vmware")
                        || l.contains("qemu")
                        || l.contains("virtualbox")
                });

                if has_nvidia && (has_intel || has_amd) {
                    let prim = if has_intel {
                        "Intel UHD/Iris"
                    } else {
                        "AMD Radeon"
                    };
                    return DetectedGpu::Hybrid {
                        primary: prim.to_string(),
                        discrete: "NVIDIA Discrete GPU".to_string(),
                    };
                } else if has_nvidia {
                    return DetectedGpu::Nvidia("NVIDIA GeForce/Quadro".to_string());
                } else if has_amd {
                    return DetectedGpu::Amd("AMD Radeon Graphics".to_string());
                } else if has_intel {
                    return DetectedGpu::Intel("Intel Graphics".to_string());
                } else if has_vm {
                    return DetectedGpu::VirtualMachine("Virtual Display (QEMU/VirtIO)".to_string());
                }
            }
        }
    }

    // 2. Fallback: inspect /sys/class/drm/card*/device/vendor
    let mut has_nvidia = false;
    let mut has_amd = false;
    let mut has_intel = false;
    let mut has_vm = false;

    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let vendor_path = entry.path().join("device/vendor");
            if let Ok(v) = fs::read_to_string(vendor_path) {
                let v = v.trim().to_lowercase();
                if v == "0x10de" {
                    has_nvidia = true;
                } else if v == "0x1002" {
                    has_amd = true;
                } else if v == "0x8086" {
                    has_intel = true;
                } else if v == "0x1af4" || v == "0x15ad" || v == "0x1ab8" {
                    has_vm = true;
                }
            }
        }
    }

    if has_nvidia && (has_intel || has_amd) {
        let prim = if has_intel { "Intel" } else { "AMD" };
        DetectedGpu::Hybrid {
            primary: prim.to_string(),
            discrete: "NVIDIA".to_string(),
        }
    } else if has_nvidia {
        DetectedGpu::Nvidia("NVIDIA GPU".to_string())
    } else if has_amd {
        DetectedGpu::Amd("AMD Radeon GPU".to_string())
    } else if has_intel {
        DetectedGpu::Intel("Intel Integrated Graphics".to_string())
    } else if has_vm {
        DetectedGpu::VirtualMachine("Virtual Display".to_string())
    } else {
        DetectedGpu::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistroInfo {
    pub family: DistroFamily,
    pub id: String,
    pub pretty_name: String,
    pub id_like: Vec<String>,
    pub is_compatible: bool,
    pub is_arch_compatible: bool,
    pub is_fedora_compatible: bool,
    pub is_nixos_compatible: bool,
    pub distro_flavor: String,
    pub arch_flavor: String,
    pub aur_helper: Option<String>,
    pub pacman_available: bool,
    pub dnf_available: bool,
    pub nix_available: bool,
    pub nixos_rebuild_available: bool,
    pub flatpak_available: bool,
    pub kernel: String,
    pub arch: String,
    pub current_user: String,
    pub is_root: bool,
    pub home_free_gb: f64,
    pub total_ram_gb: f64,
    pub available_ram_gb: f64,
    pub cpu_cores: usize,
    pub cpu_model: String,
    pub is_uefi: bool,
    pub gpu: DetectedGpu,
    pub git_available: bool,
    pub cargo_available: bool,
    pub hyprland_running: bool,
    pub is_laptop: bool,
    pub battery_percent: Option<u8>,
    pub is_btrfs: bool,
}

impl DistroInfo {
    pub fn detect() -> Self {
        let (id, pretty_name, id_like, version_id) = read_os_release();

        let pacman_available = which("pacman").is_ok();
        let dnf_available = which("dnf").is_ok() || which("rpm").is_ok();
        let nix_available = which("nix").is_ok();
        let nixos_rebuild_available = which("nixos-rebuild").is_ok();
        let flatpak_available = which("flatpak").is_ok();

        let is_nix = is_nixos_family(&id, &id_like);
        let is_fed = is_fedora_family(&id, &id_like) || Path::new("/etc/fedora-release").exists();
        let is_arch = is_arch_family(&id, &id_like) || Path::new("/etc/arch-release").exists();

        let family = if is_nix {
            DistroFamily::NixOS
        } else if is_fed || (dnf_available && !pacman_available) {
            DistroFamily::Fedora
        } else if is_arch || pacman_available {
            DistroFamily::Arch
        } else if nix_available || nixos_rebuild_available {
            DistroFamily::NixOS
        } else if dnf_available {
            DistroFamily::Fedora
        } else {
            DistroFamily::Unknown
        };

        let is_compatible = family != DistroFamily::Unknown;
        let is_arch_compatible = family == DistroFamily::Arch;
        let is_fedora_compatible = family == DistroFamily::Fedora;
        let is_nixos_compatible = family == DistroFamily::NixOS;

        let distro_flavor = match family {
            DistroFamily::Arch => detect_arch_flavor(&id, &pretty_name, true),
            DistroFamily::Fedora => detect_fedora_flavor(&id, &pretty_name, &version_id),
            DistroFamily::NixOS => detect_nixos_flavor(&pretty_name, &version_id),
            DistroFamily::Unknown => pretty_name.clone(),
        };
        let arch_flavor = distro_flavor.clone();

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
        let cpu_model = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Multi-core Processor".to_string());

        let is_uefi = is_uefi_mode();
        let gpu = detect_gpu();

        let git_available = which("git").is_ok();
        let cargo_available = which("cargo").is_ok();
        let hyprland_running = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some()
            || std::env::var("XDG_CURRENT_DESKTOP")
                .map(|v| v.to_lowercase().contains("hyprland"))
                .unwrap_or(false);

        let (is_laptop, battery_percent) = {
            let mut found_laptop = false;
            let mut batt_pct = None;
            if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("BAT") {
                        found_laptop = true;
                        if let Ok(cap_str) = fs::read_to_string(entry.path().join("capacity")) {
                            if let Ok(val) = cap_str.trim().parse::<u8>() {
                                batt_pct = Some(val);
                            }
                        }
                        break;
                    }
                }
            }
            (found_laptop, batt_pct)
        };

        let is_btrfs = fs::read_to_string("/proc/mounts")
            .map(|content| {
                content.lines().any(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    parts.len() >= 3 && (parts[1] == "/" || parts[1] == "/home") && parts[2] == "btrfs"
                })
            })
            .unwrap_or(false);

        Self {
            family,
            id,
            pretty_name,
            id_like,
            is_compatible,
            is_arch_compatible,
            is_fedora_compatible,
            is_nixos_compatible,
            distro_flavor,
            arch_flavor,
            aur_helper,
            pacman_available,
            dnf_available,
            nix_available,
            nixos_rebuild_available,
            flatpak_available,
            kernel,
            arch,
            current_user,
            is_root,
            home_free_gb,
            total_ram_gb,
            available_ram_gb,
            cpu_cores,
            cpu_model,
            is_uefi,
            gpu,
            git_available,
            cargo_available,
            hyprland_running,
            is_laptop,
            battery_percent,
            is_btrfs,
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
        "chimeraos" => "ChimeraOS (Arch derivative)".to_string(),
        "blendos" => "BlendOS".to_string(),
        "rebornos" => "RebornOS".to_string(),
        _ => {
            if pretty_name.to_lowercase().contains("arch") {
                pretty_name.to_string()
            } else {
                format!("{pretty_name} (Arch-compatible)")
            }
        }
    }
}

pub fn detect_fedora_flavor(id: &str, pretty_name: &str, version_id: &str) -> String {
    let ver_suffix = if !version_id.is_empty() {
        format!(" {version_id}")
    } else {
        String::new()
    };

    match id {
        "fedora" => format!("Fedora Linux{ver_suffix}"),
        "nobara" => format!("Nobara Linux{ver_suffix}"),
        "bazzite" => format!("Bazzite{ver_suffix}"),
        "ultramarine" => format!("Ultramarine Linux{ver_suffix}"),
        "rhel" => format!("Red Hat Enterprise Linux{ver_suffix}"),
        "centos" => format!("CentOS Stream{ver_suffix}"),
        "rocky" => format!("Rocky Linux{ver_suffix}"),
        "almalinux" => format!("AlmaLinux{ver_suffix}"),
        "silverblue" => format!("Fedora Silverblue{ver_suffix}"),
        "kinoite" => format!("Fedora Kinoite{ver_suffix}"),
        "sericea" => format!("Fedora Sericea{ver_suffix}"),
        "onyx" => format!("Fedora Onyx{ver_suffix}"),
        _ => {
            if pretty_name.to_lowercase().contains("fedora") {
                pretty_name.to_string()
            } else {
                format!("{pretty_name} (Fedora-compatible)")
            }
        }
    }
}

pub fn detect_nixos_flavor(pretty_name: &str, version_id: &str) -> String {
    if !version_id.is_empty() {
        format!("NixOS {version_id}")
    } else if !pretty_name.is_empty() && pretty_name.to_lowercase().contains("nixos") {
        pretty_name.to_string()
    } else {
        "NixOS (Flakes)".to_string()
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
        "blendos",
        "chimeraos",
        "rebornos",
        "anarchy",
    ];

    if arch_ids.iter().any(|&candidate| candidate == id) {
        return true;
    }

    id_like.iter().any(|like| like == "arch")
}

pub fn is_arch_family(id: &str, id_like: &[String]) -> bool {
    is_arch_id(id, id_like) || Path::new("/etc/arch-release").exists()
}

pub fn is_fedora_family(id: &str, id_like: &[String]) -> bool {
    let fedora_ids = [
        "fedora",
        "nobara",
        "bazzite",
        "ultramarine",
        "rhel",
        "centos",
        "rocky",
        "almalinux",
        "korora",
        "chapeau",
        "qubes",
        "silverblue",
        "kinoite",
        "sericea",
        "onyx",
    ];

    if fedora_ids.iter().any(|&candidate| candidate == id) {
        return true;
    }

    id_like
        .iter()
        .any(|like| like == "fedora" || like == "rhel" || like == "centos")
}

pub fn is_nixos_family(id: &str, id_like: &[String]) -> bool {
    id == "nixos"
        || id_like.iter().any(|like| like == "nixos")
        || Path::new("/etc/NIXOS").exists()
        || which("nixos-rebuild").is_ok()
}

fn read_os_release() -> (String, String, Vec<String>, String) {
    let mut id = "unknown".to_string();
    let mut pretty_name = "Linux".to_string();
    let mut id_like = Vec::new();
    let mut version_id = String::new();

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
                        "VERSION_ID" => version_id = v,
                        _ => {}
                    }
                }
            }
            break;
        }
    }

    (id, pretty_name, id_like, version_id)
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
        assert!(is_arch_id("chimeraos", &[]));
        assert!(is_arch_id("blendos", &[]));
        assert!(is_arch_id("rebornos", &[]));

        let ubuntu_like = vec!["debian".to_string()];
        assert!(!is_arch_id("ubuntu", &ubuntu_like));
        assert!(!is_arch_id("fedora", &[]));
    }

    #[test]
    fn test_fedora_family_detection() {
        assert!(is_fedora_family("fedora", &[]));
        assert!(is_fedora_family("nobara", &["fedora".to_string()]));
        assert!(is_fedora_family("bazzite", &["fedora".to_string()]));
        assert!(is_fedora_family("ultramarine", &[]));
        assert!(is_fedora_family("rhel", &[]));
        assert!(is_fedora_family("centos", &[]));
        assert!(is_fedora_family("rocky", &[]));
        assert!(is_fedora_family("almalinux", &[]));
        assert!(is_fedora_family("silverblue", &[]));
        assert!(is_fedora_family("kinoite", &[]));
        assert!(is_fedora_family("sericea", &[]));
        assert!(is_fedora_family("onyx", &[]));
        assert!(!is_fedora_family("arch", &[]));
        assert!(!is_fedora_family("nixos", &[]));
    }

    #[test]
    fn test_nixos_family_detection() {
        assert!(is_nixos_family("nixos", &[]));
        assert!(is_nixos_family("custom", &["nixos".to_string()]));
        assert!(!is_nixos_family("fedora", &[]));
        assert!(!is_nixos_family("arch", &[]));
    }

    #[test]
    fn test_flavor_names() {
        assert_eq!(detect_arch_flavor("cachyos", "CachyOS Linux", true), "CachyOS Linux");
        assert_eq!(detect_arch_flavor("arch", "Arch Linux", true), "Arch Linux (Pure)");
        assert_eq!(detect_arch_flavor("endeavouros", "EndeavourOS", true), "EndeavourOS");
        assert_eq!(detect_arch_flavor("garuda", "Garuda Linux", true), "Garuda Linux");
        assert_eq!(detect_arch_flavor("steamos", "SteamOS", true), "SteamOS (Arch derivative)");
        assert_eq!(detect_arch_flavor("chimeraos", "ChimeraOS", true), "ChimeraOS (Arch derivative)");
        assert_eq!(detect_fedora_flavor("fedora", "Fedora Linux", "41"), "Fedora Linux 41");
        assert_eq!(detect_fedora_flavor("nobara", "Nobara Linux", "40"), "Nobara Linux 40");
        assert_eq!(detect_fedora_flavor("bazzite", "Bazzite", "41"), "Bazzite 41");
        assert_eq!(detect_fedora_flavor("silverblue", "Fedora Silverblue", "41"), "Fedora Silverblue 41");
        assert_eq!(detect_nixos_flavor("NixOS", "24.11"), "NixOS 24.11");
    }

    #[test]
    fn test_distro_family_build_steps() {
        let config = crate::model::config::InstallConfig::default();
        let arch_steps = DistroFamily::Arch.build_steps(&config);
        let fedora_steps = DistroFamily::Fedora.build_steps(&config);
        let nixos_steps = DistroFamily::NixOS.build_steps(&config);

        assert!(!arch_steps.is_empty(), "Arch steps should not be empty");
        assert!(!fedora_steps.is_empty(), "Fedora steps should not be empty");
        assert!(!nixos_steps.is_empty(), "NixOS steps should not be empty");

        assert!(arch_steps.iter().any(|s| s.id == "arch_packages"));
        assert!(fedora_steps.iter().any(|s| s.id == "fedora_packages"));
        assert!(nixos_steps.iter().any(|s| s.id == "nixos_rebuild_switch"));
    }

    #[test]
    fn test_detected_gpu_labels() {
        let nvidia = DetectedGpu::Nvidia("RTX 4080".to_string());
        assert!(nvidia.is_nvidia());
        assert!(nvidia.label().contains("NVIDIA"));

        let hybrid = DetectedGpu::Hybrid {
            primary: "Intel".to_string(),
            discrete: "NVIDIA".to_string(),
        };
        assert!(hybrid.is_nvidia());
        assert!(hybrid.label().contains("Hybrid"));
    }
}
