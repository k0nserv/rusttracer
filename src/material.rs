use color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub reflection: f64,
}

impl Material {
    pub fn new(ambient_color: Color, diffuse_color: Color, reflection: f64) -> Material {
        Material {
            ambient_color: ambient_color,
            diffuse_color: diffuse_color,
            reflection: reflection,
        }
    }
}
