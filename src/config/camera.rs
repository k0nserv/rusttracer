#[derive(Deserialize, Debug)]
pub struct Camera {
    pub fov: f64,
    pub width: u32,
    pub height: u32,
    pub position: [f64; 3],
    pub look_at: [f64; 3],
    pub up: [f64; 3],
}

impl Camera {
    pub fn new(fov: f64,
               width: u32,
               height: u32,
               position: [f64; 3],
               look_at: [f64; 3],
               up: [f64; 3])
               -> Camera {
        Camera {
            fov: fov,
            width: width,
            height: height,
            position: position,
            look_at: look_at,
            up: up,
        }
    }
}
