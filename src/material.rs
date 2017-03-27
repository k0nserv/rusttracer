use color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub reflection_coefficient: Option<f64>,
    pub refraction_coefficient: Option<f64>,
}

impl Material {
    pub fn new(ambient_color: Color,
               diffuse_color: Color,
               specular_color: Color,
               reflection_coefficient: Option<f64>,
               refraction_coefficient: Option<f64>)
               -> Material {
        Material {
            ambient_color: ambient_color,
            diffuse_color: diffuse_color,
            specular_color: specular_color,
            reflection_coefficient: reflection_coefficient,
            refraction_coefficient: refraction_coefficient,
        }
    }

    pub fn is_reflective(&self) -> bool {
        self.reflection_coefficient.is_some()
    }

    pub fn is_refractive(&self) -> bool {
        self.refraction_coefficient.is_some()
    }
}
