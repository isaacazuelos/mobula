use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub depth: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 400,
            height: 300,
            samples: 8,
            depth: 8,
        }
    }
}
