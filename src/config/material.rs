use material::IllumninationModel;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Material {
    pub ambient_color: [f64; 3],
    pub diffuse_color: [f64; 3],
    pub specular_color: [f64; 3],
    pub specular_exponent: f64,
    pub illumination_model: IllumninationModel,
    pub reflection_coefficient: Option<f64>,
    pub refraction_coefficient: Option<f64>,
}
