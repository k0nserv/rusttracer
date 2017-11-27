use config::Transform;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
    Sphere {
        radius: f64,
        transforms: Option<Vec<Transform>>,
        material_id: Option<usize>,
    },
    Plane {
        normal: [f64; 3],
        transforms: Option<Vec<Transform>>,
        material_id: Option<usize>,
    },
    Mesh {
        path: String,
        transforms: Option<Vec<Transform>>,
    },
}
