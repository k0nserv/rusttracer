use serde::Deserialize;

use super::Transform;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
    Sphere {
        radius: f32,
        transforms: Option<Vec<Transform>>,
        material_name: Option<String>,
    },
    Plane {
        normal: [f32; 3],
        transforms: Option<Vec<Transform>>,
        material_name: Option<String>,
    },
    Mesh {
        path: String,
        transforms: Option<Vec<Transform>>,
        material_name: Option<String>,
    },
    MeshInstance {
        path: String,
        transforms: Option<Vec<Transform>>,
        material_name: Option<String>,
    },
}
