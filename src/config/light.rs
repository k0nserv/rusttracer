use crate::light::Falloff;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
    PointLight {
        origin: [f32; 3],
        color: [f32; 3],
        intensity: f32,
        falloff: Option<Falloff>,
        specular: Option<bool>,
        diffuse: Option<bool>,
    },
    DirectionalLight {
        direction: [f32; 3],
        color: [f32; 3],
        intensity: f32,
        specular: Option<bool>,
        diffuse: Option<bool>,
    },
}
