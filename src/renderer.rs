extern crate crossbeam;

use color::Color;
use scene::Scene;
use camera::Camera;
use ray::Ray;
use intersection::Intersection;
use material::Material;
use math::EPSILON;

use std::ops::Range;
use std::sync::mpsc;

pub enum SuperSampling {
    Off,
    On(u32),
}

pub struct Renderer<'a> {
    scene: &'a Scene<'a>,
    camera: &'a Camera,
    super_sampling: SuperSampling,
    num_threads: u32,
}

unsafe impl<'a> Sync for Renderer<'a> {}
unsafe impl<'a> Send for Renderer<'a> {}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene<'a>,
               camera: &'a Camera,
               super_sampling: SuperSampling,
               num_threads: u32)
               -> Renderer<'a> {
        assert!(camera.height % num_threads == 0,
                "camera.height should be devisble by num_threads");

        Renderer {
            scene: scene,
            camera: camera,
            super_sampling: super_sampling,
            num_threads: num_threads,
        }
    }

    pub fn render(&self, max_depth: u32) -> Vec<Color> {
        if self.num_threads == 1 {
            let range: Range<usize> = (0 as usize)..(self.camera.height as usize);
            self.render_segment(0, &range, max_depth)
        } else {
            let thread_segments = self.segments();
            let mut rxs = vec![];

            crossbeam::scope(|scope| {
                rxs = thread_segments.iter()
                    .map(|&(start, end, offset)| {
                        let (tx, rx) = mpsc::channel();
                        let range = start..end;

                        scope.spawn(move || {
                                        let _ = tx.send(self.render_segment(offset, &range, max_depth));
                                    });
                        rx
                    })
                    .collect::<Vec<_>>();
            });


            rxs.iter().flat_map(|rx| rx.recv().unwrap()).collect()
        }
    }

    fn segments(&self) -> Vec<(usize, usize, usize)> {
        let segment_size = self.camera.height / self.num_threads;
        let mut ranges = Vec::with_capacity(self.num_threads as usize);

        for i in 0..self.num_threads {
            let start = (0 * segment_size) as usize;
            let end = ((0 + 1) * segment_size) as usize;
            ranges.push((start, end, (segment_size * i) as usize));
        }

        ranges
    }

    fn render_segment(&self,
                      segment_offset: usize,
                      segment_range: &Range<usize>,
                      max_depth: u32)
                      -> Vec<Color> {
        let width = self.camera.width as usize;
        let height = &segment_range.end - &segment_range.start;
        let mut colors = vec![Color::black(); width  * height];

        for y in segment_range.clone() {
            for x in 0..width {
                let samples = match self.super_sampling {
                    SuperSampling::Off => 1,
                    SuperSampling::On(samples) => samples,
                };

                let mut sample_colors = vec![Color::black(); (samples * samples) as usize];
                let global_y = segment_offset + y;

                for x_sample in 0..samples {
                    for y_sample in 0..samples {
                        let ray = self.camera.create_ray(x as u32,
                                                         self.camera.height - global_y as u32,
                                                         x_sample,
                                                         y_sample,
                                                         samples);
                        sample_colors[(y_sample * samples + x_sample) as usize] =
                            self.trace(ray, max_depth);
                    }
                }


                let index = y * width + x;

                let mut sum_r: f64 = 0.0;
                let mut sum_g: f64 = 0.0;
                let mut sum_b: f64 = 0.0;

                for color in &sample_colors {
                    sum_r += color.r_f64();
                    sum_g += color.g_f64();
                    sum_b += color.b_f64();
                }

                colors[index as usize] = Color::new_f64(sum_r / sample_colors.len() as f64,
                                                        sum_g / sample_colors.len() as f64,
                                                        sum_b / sample_colors.len() as f64);
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
            result = self.shade(&hit, ray);

            if hit.shape.material().is_reflective() {
                result = result + self.reflect(&hit, ray, depth);
            }

            if hit.shape.material().is_refractive() {
                result = result + self.refract(&hit, ray, depth);
            }
        }

        result
    }

    fn shade(&self, intersection: &Intersection, original_ray: Ray) -> Color {
        let material: &Material = intersection.shape.material();
        let mut result = material.ambient_color;

        for light in self.scene.lights {
            let mut in_shadow = false;
            let distance_to_light = (intersection.point - light.origin).length();
            let light_direction = (light.origin - intersection.point).normalize();
            let ray = Ray::new(intersection.point + light_direction * EPSILON,
                               light_direction,
                               Some(original_ray.medium_refraction));

            for object in self.scene.objects {
                if let Some(hit) = object.intersect(ray) {
                    if hit.t < distance_to_light {
                        in_shadow = true;
                        break;
                    }
                }
            }

            if in_shadow {
                continue;
            }

            let light_color = light.color();
            let mut dot = light_direction.dot(&intersection.normal);

            // Diffuse
            if dot > 0.0 {
                result = result + (light_color * material.diffuse_color) * dot;
            }

            dot = original_ray.direction.dot(&light_direction.reflect(&intersection.normal));

            // Specular
            if dot > 0.0 {
                let spec = dot.powf(20.0);

                result = result + (light_color * material.specular_color) * spec;
            }
        }

        result
    }

    fn reflect(&self, intersection: &Intersection, original_ray: Ray, current_depth: u32) -> Color {
        let new_direction = original_ray.direction.reflect(&intersection.normal).normalize();

        let new_ray = Ray::new(intersection.point + new_direction * EPSILON,
                               new_direction,
                               Some(original_ray.medium_refraction));

        let reflected_color = self.trace(new_ray, current_depth - 1);

        reflected_color *
        intersection.shape
            .material()
            .reflection_coefficient
            .unwrap_or(0.0)
    }

    fn refract(&self, intersection: &Intersection, original_ray: Ray, current_depth: u32) -> Color {
        assert!(intersection.shape
                    .material()
                    .refraction_coefficient
                    .is_some(),
                "Don't call refract for materials that aren't refractive");
        let mut refraction_coefficient = intersection.shape
            .material()
            .refraction_coefficient
            .unwrap_or(1.0);

        if intersection.inside {
            // Leaving refractive material
            refraction_coefficient = 1.0;
        }

        let n = original_ray.medium_refraction / refraction_coefficient;
        let normal = match intersection.inside {
            true => -intersection.normal,
            false => intersection.normal,
        };

        let cos_i = normal.dot(&original_ray.direction);
        let c2 = 1.0 - n * (1.0 - cos_i * cos_i);

        if c2 > 0.0 {
            let t = (original_ray.direction * n + normal * (n * cos_i - c2.sqrt())).normalize();

            let new_ray = Ray::new(intersection.point + t * EPSILON,
                                   t,
                                   Some(refraction_coefficient));

            let refraction_color = self.trace(new_ray, current_depth - 1);
            let absorbance = intersection.shape.material().ambient_color * 0.15 * -intersection.t;
            let transparency = Color::new_f64(absorbance.r_f64().exp(),
                                              absorbance.g_f64().exp(),
                                              absorbance.b_f64().exp());

            return refraction_color * transparency;


        }

        return Color::black();
    }
}
