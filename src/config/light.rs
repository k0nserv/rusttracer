use light::Falloff;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
    PointLight {
        origin: [f32; 3],
        color: [f32; 3],
        intensity: f32,
        falloff: Option<Falloff>,
    },
    DirectionalLight {
        direction: [f32; 3],
        color: [f32; 3],
        intensity: f32,
    },
}
