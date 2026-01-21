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
}

fn default_maximized() -> bool {
    false
}

impl Default for ConfigOnWindowFace {
    fn default() -> Self {
        ConfigOnWindowFace {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            x: 0.0,
            y: 0.0,
            maximized: false,
        }
    }
}
