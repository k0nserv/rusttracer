use color::Color;
use scene::Scene;
use camera::Camera;
use ray::Ray;

pub struct Renderer<'a> {
    scene: &'a Scene<'a>,
    camera: &'a Camera,
}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene<'a>, camera: &'a Camera) -> Renderer<'a> {
        Renderer {
            scene: scene,
            camera: camera,
        }
    }

    pub fn render(&self, max_depth: u32) -> Vec<Color> {
        let height = self.camera.height;
        let width = self.camera.width;
        let mut colors = vec![Color::black(); (width * height) as usize];

        for y in 0..height {
            for x in 0..width {
                let index = (height - 1 - y) * width + x;
                let ray = self.camera.create_ray(x, y);
                colors[index as usize] = self.trace(ray, max_depth);
            }
        }


        colors
    }

    fn trace(&self, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let mut result = self.scene.clear_color;
        let possible_hit = self.scene.intersect(ray);

        if let Some(hit) = possible_hit {
            result = hit.shape.material().ambient_color;
        }

        result
    }
}
