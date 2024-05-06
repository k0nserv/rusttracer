use crate::material::IllumninationModel;

#[derive(Deserialize, Debug, Clone)]
pub enum Texture {
    #[serde(rename = "name")]
    Named(String),
}

pub type OptionalTexture = Option<Texture>;

#[derive(Deserialize, Debug, Clone)]
pub struct Material {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub ambient_texture: OptionalTexture,
    pub diffuse_color: [f32; 3],
    pub diffuse_texture: OptionalTexture,
    pub specular_color: [f32; 3],
    pub specular_texture: OptionalTexture,
    pub specular_exponent: f32,
    pub illumination_model: IllumninationModel,
    pub reflection_coefficient: Option<f32>,
    pub refraction_coefficient: Option<f32>,
}
