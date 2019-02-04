use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_width")]
    pub width: u32,
    #[serde(default = "Config::default_height")]
    pub height: u32,
    #[serde(default = "Config::default_depth")]
    pub depth: u32,
    #[serde(default = "Config::default_samples")]
    pub samples: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            height: 300,
            width: 400,
            depth: Config::DEFAULT_DEPTH,
            samples: 8,
        }
    }
}

// This whole impl block is just boilerplate for serde defaults.
impl Config {
    const DEFAULT_HEIGHT: u32 = 300;
    const DEFAULT_WIDTH: u32 = 400;
    const DEFAULT_DEPTH: u32 = 8;
    const DEFAULT_SAMPLES: u32 = 8;

    fn default_height() -> u32 {
        Config::DEFAULT_HEIGHT
    }
    
    fn default_width() -> u32 {
        Config::DEFAULT_WIDTH
    }

    fn default_depth() -> u32 {
        Config::DEFAULT_DEPTH
    }

    fn default_samples() -> u32 {
        Config::DEFAULT_SAMPLES
    }
}