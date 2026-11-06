// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuroraTheme {
    pub name: &'static str,
    pub description: &'static str,
    pub accent_color_hex: &'static str,
    pub palette: [&'static str; 4],
    pub repo_url: Option<&'static str>,
}

impl AuroraTheme {
    pub fn palette_colors(&self) -> [Color; 4] {
        [
            hex_to_color(self.palette[0]),
            hex_to_color(self.palette[1]),
            hex_to_color(self.palette[2]),
            hex_to_color(self.palette[3]),
        ]
    }
}

pub fn hex_to_color(hex: &str) -> Color {
    let clean = hex.trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(125);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(207);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(255);
        Color::Rgb(r, g, b)
    } else {
        Color::Cyan
    }
}

pub const AVAILABLE_THEMES: &[AuroraTheme] = &[
    AuroraTheme {
        name: "Aurora Default",
        description: "The official signature look with vibrant pastel blues, magentas, and dark accents",
        accent_color_hex: "#7dcfff",
        palette: ["#7dcfff", "#bb9af7", "#7aa2f7", "#1a1b26"],
        repo_url: None,
    },
    AuroraTheme {
        name: "Vanta Black",
        description: "Ultra-dark minimalist stealth palette designed for zero distractions and OLED displays",
        accent_color_hex: "#43465a",
        palette: ["#585b70", "#7f849c", "#a6adc8", "#0b0b0f"],
        repo_url: Some("https://github.com/TheAhumMaitra/Vanta-Black-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Osaka Jade",
        description: "Elegant Japanese aesthetics blending rich jade greens, bamboo teals, and soft whites",
        accent_color_hex: "#56949f",
        palette: ["#56949f", "#9ccfd8", "#ebbcba", "#191724"],
        repo_url: Some("https://github.com/TheAhumMaitra/Osaka-Jade-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Everblush",
        description: "Cozy muted retro theme with soothing slate tones and earthy sage highlights",
        accent_color_hex: "#8ccf7e",
        palette: ["#8ccf7e", "#67b0e8", "#e5c76b", "#141b1e"],
        repo_url: Some("https://github.com/TheAhumMaitra/Everblush-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Kanagawa Dragon",
        description: "Dusk-lit dragon aesthetic inspired by traditional Japanese Woodblock prints",
        accent_color_hex: "#c4746e",
        palette: ["#c4746e", "#8a9a86", "#8ba4b0", "#181616"],
        repo_url: Some("https://github.com/TheAhumMaitra/Kanagawa-Dragon-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Your Name",
        description: "Breathtaking twilight and celestial twilight tones inspired by Makoto Shinkai's masterpiece",
        accent_color_hex: "#e0af68",
        palette: ["#e0af68", "#f7768e", "#7aa2f7", "#1f2335"],
        repo_url: Some("https://github.com/TheAhumMaitra/Your-Name-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Dark Decay",
        description: "Warm, cozy low-contrast dark decay palette designed for late night programming",
        accent_color_hex: "#bb9af7",
        palette: ["#bb9af7", "#7dcfff", "#e0af68", "#101014"],
        repo_url: Some("https://github.com/TheAhumMaitra/DarkDecayForAurora.git"),
    },
];
