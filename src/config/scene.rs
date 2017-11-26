use super::object::Object;
use super::light::Light;

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub clear_color: [f64; 3],
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}
