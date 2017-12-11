use ray::Ray;
use math::{Matrix4, Point3, Vector3};
use config;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    widthf: f32,
    heightf: f32,
    scale: f32,
    aspect_ratio: f32,
    camera_to_world: Matrix4,
}

impl Camera {
    pub fn new(
        fov: f32,
        width: u32,
        height: u32,
        position: Point3,
        look_at: Point3,
        tmp_up: Vector3,
    ) -> Camera {
        let aspect_ratio = (width as f32) / (height as f32);
        let scale = (fov * 0.5).tan();
        let direction = (position - look_at).normalize();
        let right = tmp_up.normalize().cross(&direction);
        let up = direction.cross(&right);

        Camera {
            width,
            height,
            widthf: (width as f32),
            heightf: (height as f32),
            scale,
            aspect_ratio,
            camera_to_world: Self::camera_to_world_matrix(
                right.normalize(),
                up.normalize(),
                direction,
                position,
            ),
        }
    }

    pub fn from_config(camera: &config::Camera) -> Camera {
        Self::new(
            camera.fov,
            camera.width,
            camera.height,
            Point3::new_from_slice(camera.position),
            Point3::new_from_slice(camera.look_at),
            Vector3::new_from_slice(camera.up),
        )
    }

    pub fn create_ray(&self, x: u32, y: u32, x_sample: u32, y_sample: u32, samples: u32) -> Ray {
        let samplesf = samples as f32;
        let sample_width = self.widthf * samplesf;
        let sample_height = self.heightf * samplesf;

        let mut x_sample_offset = x_sample as f32;
        let mut y_sample_offset = y_sample as f32;

        if samples == 1 {
            x_sample_offset = 0.5;
            y_sample_offset = 0.5;
        }

        let px = ((2.0 * (((x * samples) as f32) + x_sample_offset) / sample_width) - 1.0)
            * self.aspect_ratio * self.scale;
        let py =
            ((2.0 * (((y * samples) as f32) + y_sample_offset) / sample_height) - 1.0) * self.scale;

        let direction = Vector3::new(px, py, -1.0) * self.camera_to_world;
        let origin = Point3::at_origin() * self.camera_to_world;

        Ray::new(origin, direction.normalize(), None)
    }

    pub fn camera_to_world_matrix(
        right: Vector3,
        up: Vector3,
        direction: Vector3,
        position: Point3,
    ) -> Matrix4 {
        let mut result = Matrix4::identity();

        // right
        result[(0, 0)] = right.x;
        result[(0, 1)] = right.y;
        result[(0, 2)] = right.z;

        // up
        result[(1, 0)] = up.x;
        result[(1, 1)] = up.y;
        result[(1, 2)] = up.z;

        // direction
        result[(2, 0)] = direction.x;
        result[(2, 1)] = direction.y;
        result[(2, 2)] = direction.z;

        // position
        result[(3, 0)] = position.x;
        result[(3, 1)] = position.y;
        result[(3, 2)] = position.z;

        println!("Camera to world matrix {:?}", result);

        result
    }
}
