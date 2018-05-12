use super::light::Light;
use super::object::Object;

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub clear_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}
