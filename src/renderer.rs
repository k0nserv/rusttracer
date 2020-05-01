use camera::Camera;
use color::Color;
use intersection::Intersection;
use material::{IllumninationModel, Material};
use math::{Vector3, EPSILON};
use ray::Ray;
use scene::Scene;

use rayon::prelude::*;
use std::ops::Range;

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum SuperSampling {
    Off,
    On(u32),
}

pub struct Renderer {
    scene: Scene,
    camera: Camera,
    super_sampling: SuperSampling,
}

pub struct RefractionProperties {
    n1: f32,
    n2: f32,
    n: f32,
    cos_i: f32,
    c2: f32,
    normal: Vector3,
}

impl RefractionProperties {
    fn new(intersection: &Intersection, original_ray: &Ray) -> RefractionProperties {
        let refraction_coefficient = if intersection.inside {
            1.0
        } else {
            intersection
                .shape
                .material()
                .refraction_coefficient
                .unwrap_or(1.0)
        };

        let n = original_ray.medium_refraction / refraction_coefficient;
        let normal = if intersection.inside {
            -intersection.normal
        } else {
            intersection.normal
        };

        let cos_i = normal.dot(&original_ray.direction);
        let c2 = 1.0 - n * n * (1.0 - cos_i * cos_i);

        RefractionProperties {
            n1: original_ray.medium_refraction,
            n2: refraction_coefficient,
            n,
            cos_i,
            c2,
            normal,
        }
    }

    fn total_internal_reflection(&self) -> bool {
        self.c2 < 0.0
    }
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    pub fn new(scene: Scene, camera: Camera, super_sampling: SuperSampling) -> Renderer {
        Renderer {
            scene,
            camera,
            super_sampling,
        }
    }

    pub fn render(&self, max_depth: u32) -> Vec<u8> {
        let range: Range<usize> = (0 as usize)..(self.camera.height as usize);
        let width = self.camera.width as usize;

        range
            .into_par_iter()
            .flat_map(|y| {
                (0..width)
                    .flat_map(move |x| self.render_point(max_depth, x, y))
                    .collect::<Vec<u8>>()
                    .into_par_iter()
            })
            .collect::<Vec<u8>>()
    }

    fn render_point(&self, max_depth: u32, x: usize, y: usize) -> Color {
        let samples = match self.super_sampling {
            SuperSampling::Off => 1,
            SuperSampling::On(samples) => samples,
        };

        // TODO: Get rid of this vec in favour of just accumulating the sums directly?
        let mut sample_colors = vec![Color::black(); (samples * samples) as usize];

        for x_sample in 0..samples {
            for y_sample in 0..samples {
                let ray = self.camera.create_ray(
                    x as u32,
                    self.camera.height - y as u32,
                    x_sample,
                    y_sample,
                    samples,
                );
                sample_colors[(y_sample * samples + x_sample) as usize] =
                    self.trace(ray, max_depth, true);
            }
        }

        let mut sum_r: f32 = 0.0;
        let mut sum_g: f32 = 0.0;
        let mut sum_b: f32 = 0.0;

        for color in &sample_colors {
            sum_r += color.r_f32();
            sum_g += color.g_f32();
            sum_b += color.b_f32();
        }

        Color::new_f32(
            sum_r / sample_colors.len() as f32,
            sum_g / sample_colors.len() as f32,
            sum_b / sample_colors.len() as f32,
        )
    }

    fn trace(&self, ray: Ray, depth: u32, cull: bool) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let mut result = self.scene.clear_color;
        let possible_hit = self.scene.intersect(ray, cull);

        if let Some(hit) = possible_hit {
            let material = hit.shape.material();

            result = match material.illumination_model {
                IllumninationModel::Constant => material.diffuse_color(hit.texture_coord),
                IllumninationModel::Diffuse => self.shade(&hit, ray, false),
                IllumninationModel::DiffuseSpecular => self.shade(&hit, ray, true),
                IllumninationModel::DiffuseSpecularReflective => {
                    self.shade(&hit, ray, true) + self.reflect(&hit, ray, depth)
                }
                IllumninationModel::DiffuseSpecularReflectiveGlass => {
                    self.shade(&hit, ray, true) + self.reflect(&hit, ray, depth)
                }
                IllumninationModel::DiffuseSpecularFresnel => {
                    self.shade(&hit, ray, true) + self.reflect(&hit, ray, depth)
                }
                IllumninationModel::DiffuseSpecularRefracted => {
                    let refraction_properties = RefractionProperties::new(&hit, &ray);

                    let mut color = self.shade(&hit, ray, true) + self.reflect(&hit, ray, depth);
                    if !refraction_properties.total_internal_reflection() {
                        color = color + self.refract(&hit, ray, depth, &refraction_properties);
                    }

                    color
                }
                IllumninationModel::DiffuseSpecularRefractedFresnel => {
                    let refraction_properties = RefractionProperties::new(&hit, &ray);
                    let kr = self.fresnel(&refraction_properties);
                    let kt = 1.0 - kr;

                    let mut color =
                        self.shade(&hit, ray, true) + self.reflect(&hit, ray, depth) * kr;
                    if !refraction_properties.total_internal_reflection() {
                        color = color + self.refract(&hit, ray, depth, &refraction_properties) * kt;
                    }

                    color
                }
            };
        }

        result
    }

    fn shade(&self, intersection: &Intersection, original_ray: Ray, specular: bool) -> Color {
        let material: &Material = intersection.shape.material();
        let mut result =
            material.ambient_color(intersection.texture_coord) * self.scene.ambient_color;

        // TODO: Move lights iteration to Scene
        for light in &self.scene.lights {
            let ray = light.create_shadow_ray(intersection, Some(original_ray.medium_refraction));
            let distance_to_light = light.distance_to_light(&intersection);
            if self
                .scene
                .first_intersection(ray, false, distance_to_light)
                .is_some()
            {
                continue;
            }

            if let Some(diffuse_color) =
                light.diffuse_color(intersection, material, distance_to_light)
            {
                result = result + diffuse_color;
            }

            // Specular
            if specular {
                if let Some(specular_color) =
                    light.specular_color(intersection, material, &original_ray, distance_to_light)
                {
                    result = result + specular_color;
                }
            }
        }

        result
    }

    fn reflect(&self, intersection: &Intersection, original_ray: Ray, current_depth: u32) -> Color {
        let new_direction = original_ray
            .direction
            .reflect(&intersection.normal)
            .normalize();

        let new_ray = Ray::new(
            (intersection.point + new_direction * EPSILON).as_point(),
            new_direction,
            Some(original_ray.medium_refraction),
        );

        let reflected_color = self.trace(new_ray, current_depth - 1, false);

        reflected_color
            * intersection
                .shape
                .material()
                .reflection_coefficient
                .unwrap_or(0.0)
    }

    fn refract(
        &self,
        intersection: &Intersection,
        original_ray: Ray,
        current_depth: u32,
        refraction_properties: &RefractionProperties,
    ) -> Color {
        assert!(
            intersection.shape.material().is_refractive(),
            "Don't call refract for materials that aren't refractive"
        );
        let (c2, n, normal, cos_i, n2) = (
            refraction_properties.c2,
            refraction_properties.n,
            refraction_properties.normal,
            refraction_properties.cos_i,
            refraction_properties.n2,
        );

        if c2 > 0.0 {
            let direction =
                (original_ray.direction * n + normal * (n * cos_i - c2.sqrt())).normalize();

            let new_ray = Ray::new(
                (intersection.point + direction * EPSILON).as_point(),
                direction,
                Some(n2),
            );

            let refraction_color = self.trace(new_ray, current_depth - 1, false);
            let absorbance = intersection
                .shape
                .material()
                .ambient_color(intersection.texture_coord)
                * 0.15
                * -intersection.t;
            let transparency = Color::new_f32(
                absorbance.r_f32().exp(),
                absorbance.g_f32().exp(),
                absorbance.b_f32().exp(),
            );

            return refraction_color * transparency;
        }

        Color::black()
    }

    fn fresnel(&self, refraction_properties: &RefractionProperties) -> f32 {
        let (n1, n2, cos_i) = (
            refraction_properties.n1,
            refraction_properties.n2,
            refraction_properties.cos_i,
        );
        let sin_t = n1 / n2 * (1.0 - cos_i * cos_i).max(0.0).sqrt();
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_i.abs();

        let rs = ((n2 * cos_i) - (n1 * cos_t)) / ((n2 * cos_i) + (n1 * cos_t));
        let rp = ((n1 * cos_t) - (n2 * cos_i)) / ((n1 * cos_t) + (n2 * cos_i));

        (rs * rs + rp * rp) / 2.0
    }
}
