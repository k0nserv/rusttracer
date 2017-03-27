use ray::Ray;
use math::Vector3;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    widthf: f64,
    heightf: f64,
    x0: f64,
    y0: f64,
}

impl Camera {
    pub fn new(fov: f64, width: u32, height: u32) -> Camera {
        let aspect_ratio = (height as f64) / (width as f64);
        let vertical_fov = fov * aspect_ratio;
        let x0 = (fov * 0.5).sin();
        let y0 = (vertical_fov * 0.5).sin();

        Camera {
            width: width,
            height: height,
            widthf: (width as f64),
            heightf: (height as f64),
            x0: x0,
            y0: y0,
        }
    }

    pub fn create_ray(&self, x: u32, y: u32, x_sample: u32, y_sample: u32, samples: u32) -> Ray {
        let samplesf = samples as f64;
        let sample_width = self.widthf * samplesf;
        let sample_height = self.heightf * samplesf;

        // Map pixel coordinates to space in -1, 1 range
        let px = ((((x * samples + x_sample) as f64) * 2.0) / sample_width) - 1.0;
        let py = ((((y * samples + y_sample) as f64) * 2.0) / sample_height) - 1.0;

        let direction = Vector3::new((px as f64) * self.x0, (py as f64) * self.y0, 1.0);

        Ray::new(Vector3::at_origin(), direction.normalize(), None)
    }
}
