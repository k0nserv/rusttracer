#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
    PointLight {
        origin: [f64; 3],
        color: [f64; 3],
        intensity: f64,
    },
}
