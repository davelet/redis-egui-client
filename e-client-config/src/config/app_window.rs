use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigOnWindowFace {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    #[serde(default = "default_maximized")]
    pub maximized: bool,
    #[serde(default = "default_hash_field_width")]
    pub hash_field_width: u32,
    #[serde(default = "default_hash_value_width")]
    pub hash_value_width: u32,
}

fn default_maximized() -> bool {
    false
}

fn default_hash_field_width() -> u32 {
    200
}

fn default_hash_value_width() -> u32 {
    400
}

impl Default for ConfigOnWindowFace {
    fn default() -> Self {
        ConfigOnWindowFace {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            x: 0.0,
            y: 0.0,
            maximized: false,
            hash_field_width: default_hash_field_width(),
            hash_value_width: default_hash_value_width(),
        }
    }
}
