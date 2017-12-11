#[derive(Deserialize, Debug)]
pub struct Camera {
    pub fov: f32,
    pub width: u32,
    pub height: u32,
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub up: [f32; 3],
}

impl Camera {
    pub fn new(
        fov: f32,
        width: u32,
        height: u32,
        position: [f32; 3],
        look_at: [f32; 3],
        up: [f32; 3],
    ) -> Camera {
        Camera {
            fov,
            width,
            height,
            position,
            look_at,
            up,
        }
    }
}
