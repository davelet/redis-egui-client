use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    #[serde(default = "default_maximized")]
    pub maximized: bool,
}

fn default_maximized() -> bool {
    false
}
