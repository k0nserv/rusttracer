use config::Transform;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
    Sphere {
        radius: f32,
        transforms: Option<Vec<Transform>>,
        material_id: Option<usize>,
    },
    Plane {
        normal: [f32; 3],
        transforms: Option<Vec<Transform>>,
        material_id: Option<usize>,
    },
    Mesh {
        path: String,
        transforms: Option<Vec<Transform>>,
    },
}
