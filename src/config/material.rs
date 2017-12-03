use material::IllumninationModel;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Material {
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub specular_exponent: f32,
    pub illumination_model: IllumninationModel,
    pub reflection_coefficient: Option<f32>,
    pub refraction_coefficient: Option<f32>,
}
