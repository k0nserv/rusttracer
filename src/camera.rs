use ray::Ray;
use math::Vector3;

pub struct Camera {
    fov: f64,
    pub width: u32,
    pub height: u32,
    widthf: f64,
    heightf: f64,
    aspect_ratio: f64,
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
            fov: fov,
            width: width,
            height: height,
            widthf: (width as f64),
            heightf: (height as f64),
            aspect_ratio: aspect_ratio,
            x0: x0,
            y0: y0,
        }
    }

    pub fn create_ray(&self, x: u32, y: u32, xSample: u32, ySample: u32, samples: u32) -> Ray {
        let samplesf = samples as f64;
        let sample_width = self.widthf * samplesf;
        let sample_height = self.heightf * samplesf;
        let px = (((x * samples + xSample) as f64) * 2.0) / sample_width - 1.0;
        let py = (((y * samples + ySample) as f64) * 2.0) / sample_height - 1.0;

        let direction = Vector3::new((px as f64) * self.x0, (py as f64) * self.y0, 1.0);

        Ray::new(Vector3::at_origin(), direction.normalize(), None)
    }
}
