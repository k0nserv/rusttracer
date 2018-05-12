#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
    PointLight {
        origin: [f32; 3],
        color: [f32; 3],
        intensity: f32,
    },
    DirectionalLight {
        direction: [f32; 3],
        color: [f32; 3],
        intensity: f32,
    },
}
