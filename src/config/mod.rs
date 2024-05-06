mod camera;
mod light;
mod material;
mod object;
mod scene;
mod transform;

pub use self::camera::Camera;
pub use self::light::Light;
pub use self::material::Material;
pub use self::material::Texture;
pub use self::object::Object;
pub use self::scene::Scene;
pub use self::transform::Transform;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::renderer::SuperSampling;

#[derive(Debug)]
pub struct ConfigError {
    cause: Option<Box<dyn Error>>,
    message: String,
}

impl ConfigError {
    fn new(message: String, cause: Option<Box<dyn Error>>) -> Self {
        ConfigError { message, cause }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            Some(ref inner_error) => write!(f, "{} with error: {}", self.message, inner_error),
            None => write!(f, "{}", self.message),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.cause {
            Some(ref error) => Some(error.as_ref()),
            None => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(cause: io::Error) -> Self {
        Self::new(
            String::from("Failed to load/read config"),
            Some(Box::new(cause)),
        )
    }
}

impl From<serde_json::error::Error> for ConfigError {
    fn from(cause: serde_json::error::Error) -> Self {
        Self::new(
            String::from("Failed to parse config"),
            Some(Box::new(cause)),
        )
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
        let mut file = File::open(path)?;

        file.read_to_string(&mut contents)?;
        let config = serde_json::from_str::<Self>(&contents)?;

        if config.scenes.is_empty() {
            return Err(ConfigError::new(
                String::from("Config should provide at least one scene, found none"),
                None,
            ));
        }

        if config.cameras.is_empty() {
            return Err(ConfigError::new(
                String::from("Config should provide at least one camera, found none"),
                None,
            ));
        }

        Ok(config)
    }
}
