use std::f32;

use color::Color;
use intersection::Intersection;
use material::Material;
use math::{Vector3, EPSILON};

use ray::Ray;

use super::Light;

pub struct Directional {
    pub direction: Vector3,
    inverse_direction: Vector3,
    pub color: Color,
    intensity: f32,
    diffuse: bool,
    specular: bool,
}

impl Directional {
    pub fn new(
        direction: Vector3,
        color: Color,
        intensity: f32,
        diffuse: bool,
        specular: bool,
    ) -> Self {
        let normalized_direction = direction.normalize();
        Self {
            direction: normalized_direction,
            inverse_direction: -normalized_direction,
            color,
            intensity,
            diffuse,
            specular,
        }
    }

    pub fn intensity(&self, _distance_to_light: f32) -> f32 {
        self.intensity
    }
}

impl Light for Directional {
    fn create_shadow_ray(
        &self,
        intersection: &Intersection,
        medium_refraction: Option<f32>,
    ) -> Ray {
        let direction = self.inverse_direction;
        Ray::new(
            (intersection.point + direction * 1e-3).as_point(),
            direction,
            medium_refraction,
        )
    }

    fn distance_to_light(&self, _intersection: &Intersection) -> f32 {
        f32::INFINITY
    }

    fn diffuse_color(
        &self,
        intersection: &Intersection,
        material: &Material,
        distance_to_light: f32,
    ) -> Option<Color> {
        if !self.diffuse {
            return None;
        }

        let dot = self.inverse_direction.dot(&intersection.normal);

        if dot > 0.0 {
            Some(
                (self.color * material.diffuse_color(intersection.texture_coord))
                    * dot
                    * self.intensity(distance_to_light),
            )
        } else {
            None
        }
    }

    fn specular_color(
        &self,
        intersection: &Intersection,
        material: &Material,
        ray: &Ray,
        distance_to_light: f32,
    ) -> Option<Color> {
        if !self.specular {
            return None;
        }

        let dot = ray
            .direction
            .dot(&self.inverse_direction.reflect(&intersection.normal));

        if dot > 0.0 {
            let spec = dot.powf(material.specular_exponent);
            Some(
                (self.color * material.specular_color(intersection.texture_coord))
                    * spec
                    * self.intensity(distance_to_light),
            )
        } else {
            None
        }
    }
}
