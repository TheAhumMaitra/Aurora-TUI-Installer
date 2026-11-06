// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuroraTheme {
    pub name: &'static str,
    pub description: &'static str,
    pub accent_color_hex: &'static str,
    pub repo_url: Option<&'static str>,
}

pub const AVAILABLE_THEMES: &[AuroraTheme] = &[
    AuroraTheme {
        name: "Aurora Default",
        description: "The official signature look with vibrant pastel blues, magentas, and dark accents",
        accent_color_hex: "#7dcfff",
        repo_url: None,
    },
    AuroraTheme {
        name: "Vanta Black",
        description: "Ultra-dark minimalist stealth palette designed for zero distractions and OLED displays",
        accent_color_hex: "#43465a",
        repo_url: Some("https://github.com/TheAhumMaitra/Vanta-Black-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Osaka Jade",
        description: "Elegant Japanese aesthetics blending rich jade greens, bamboo teals, and soft whites",
        accent_color_hex: "#56949f",
        repo_url: Some("https://github.com/TheAhumMaitra/Osaka-Jade-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Everblush",
        description: "Cozy muted retro theme with soothing slate tones and earthy sage highlights",
        accent_color_hex: "#8ccf7e",
        repo_url: Some("https://github.com/TheAhumMaitra/Everblush-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Kanagawa Dragon",
        description: "Dusk-lit dragon aesthetic inspired by traditional Japanese Woodblock prints",
        accent_color_hex: "#c4746e",
        repo_url: Some("https://github.com/TheAhumMaitra/Kanagawa-Dragon-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Your Name",
        description: "Breathtaking twilight and celestial twilight tones inspired by Makoto Shinkai's masterpiece",
        accent_color_hex: "#e0af68",
        repo_url: Some("https://github.com/TheAhumMaitra/Your-Name-For-Aurora.git"),
    },
    AuroraTheme {
        name: "Dark Decay",
        description: "Warm, cozy low-contrast dark decay palette designed for late night programming",
        accent_color_hex: "#bb9af7",
        repo_url: Some("https://github.com/TheAhumMaitra/DarkDecayForAurora.git"),
    },
];
