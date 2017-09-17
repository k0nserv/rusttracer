extern crate toml;
use renderer::SuperSampling;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub super_sampling: SuperSampling,
}

impl Config {
    pub fn new(width: u32, height: u32, max_depth: u32, super_sampling: SuperSampling) -> Self {
        Config {
            width: width,
            height: height,
            max_depth: max_depth,
            super_sampling: super_sampling,
        }
    }

    pub fn new_from_file(path: &str) -> Result<Self, &str> {
        let file = File::open(path);
        if file.is_err() {
            return Err("Failed to open config file");
        }

        let mut contents = String::new();
        let result = file.unwrap().read_to_string(&mut contents);

        if result.is_err() {
            return Err("Failed to read config file");
        }

        let config = toml::from_str(&contents);

        if config.is_err() {
            return Err("Failed to parse config");
        }

        return Ok(config.unwrap());
    }
}
