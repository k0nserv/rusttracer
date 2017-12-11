use std::fs::File;
use std::io::prelude::*;
use std::error;
use std::fmt;

use serde_json;

use renderer::SuperSampling;
use super::camera::Camera;
use super::scene::Scene;
use super::material::Material;

#[derive(Debug, Clone)]
pub struct ConfigError<'a> {
    message: &'a str,
}

impl<'a> ConfigError<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> fmt::Display for ConfigError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load scene from config")
    }
}

impl<'a> error::Error for ConfigError<'a> {
    fn description(&self) -> &str {
        "failed to load scene from config"
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub max_depth: u32,
    pub super_sampling: SuperSampling,
    pub cameras: Vec<Camera>,
    pub scenes: Vec<Scene>,
    pub materials: Vec<Material>,
}

impl Config {
    pub fn new_from_file(path: &str) -> Result<Self, ConfigError> {
        let mut contents = String::new();
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(ConfigError::new("Failed to open config file")),
        };

        if file.read_to_string(&mut contents).is_err() {
            return Err(ConfigError::new("Failed to read config file"));
        }

        let config = match serde_json::from_str::<Self>(&contents) {
            Ok(c) => c,
            Err(_) => return Err(ConfigError::new("Failed to parse config")),
        };

        if config.scenes.is_empty() {
            return Err(ConfigError::new("Config should provide at least one scene"));
        }

        if config.cameras.is_empty() {
            return Err(ConfigError::new("Config should provide at least on camera"));
        }

        Ok(config)
    }
}
