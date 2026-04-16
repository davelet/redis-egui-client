use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
    Dracula,
    Nord,
    Gruvbox,
    Monokai,
    OneDark,
    TokyoNight,
    SolarizedDark,
    SolarizedLight,
}

impl Theme {
    pub fn display_name(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::Gruvbox => "Gruvbox",
            Theme::Monokai => "Monokai",
            Theme::OneDark => "One Dark",
            Theme::TokyoNight => "Tokyo Night",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::SolarizedLight => "Solarized Light",
        }
    }

    pub fn all() -> &'static [Theme] {
        &[
            Theme::System,
            Theme::Light,
            Theme::Dark,
            Theme::Dracula,
            Theme::Nord,
            Theme::Gruvbox,
            Theme::Monokai,
            Theme::OneDark,
            Theme::TokyoNight,
            Theme::SolarizedDark,
            Theme::SolarizedLight,
        ]
    }

    pub fn to_egui_thematic_name(&self) -> &'static str {
        match self {
            Theme::System => "", // Should not be called; use is_system() check first
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::Dracula => "dracula",
            Theme::Nord => "nord",
            Theme::Gruvbox => "gruvbox_dark",
            Theme::Monokai => "monokai",
            Theme::OneDark => "one_dark",
            Theme::TokyoNight => "tokyo_night",
            Theme::SolarizedDark => "solarized_dark",
            Theme::SolarizedLight => "solarized_light",
        }
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Theme::System)
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "system" => Ok(Theme::System),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            "dracula" => Ok(Theme::Dracula),
            "nord" => Ok(Theme::Nord),
            "gruvbox" | "gruvbox_dark" => Ok(Theme::Gruvbox),
            "monokai" => Ok(Theme::Monokai),
            "one_dark" | "onedark" => Ok(Theme::OneDark),
            "tokyo_night" | "tokyonight" => Ok(Theme::TokyoNight),
            "solarized_dark" | "solarizeddark" => Ok(Theme::SolarizedDark),
            "solarized_light" | "solarizedlight" => Ok(Theme::SolarizedLight),
            _ => Err(format!("Unknown theme: {}", s)),
        }
    }
}
