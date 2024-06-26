use crate::color::Color;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::math::Point3;
use crate::ray::Ray;

use super::{Falloff, Light};

pub struct Point {
    pub origin: Point3,
    pub color: Color,
    intensity: f32,
    falloff: Falloff,
    diffuse: bool,
    specular: bool,
}

impl Point {
    pub fn new(
        origin: Point3,
        color: Color,
        intensity: f32,
        falloff: Falloff,
        diffuse: bool,
        specular: bool,
    ) -> Self {
        Self {
            origin,
            color,
            intensity,
            falloff,
            diffuse,
            specular,
        }
    }

    pub fn intensity(&self, distance_to_light: f32) -> f32 {
        match self.falloff {
            Falloff::InverseSquare => {
                1.0 / (distance_to_light * distance_to_light) * self.intensity
            }
            Falloff::InverseLinear => 1.0 / distance_to_light * self.intensity,
        }
    }
}

impl Light for Point {
    fn create_shadow_ray(
        &self,
        intersection: &Intersection,
        medium_refraction: Option<f32>,
    ) -> Ray {
        let light_direction = (self.origin - intersection.point).normalize();
        Ray::new(
            (intersection.point + light_direction * 1e-3).as_point(),
            light_direction,
            medium_refraction,
        )
    }

    fn distance_to_light(&self, intersection: &Intersection) -> f32 {
        (intersection.point - self.origin).length()
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

        let light_direction = (self.origin - intersection.point).normalize();
        let dot = light_direction.dot(&intersection.normal);

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

        let light_direction = (self.origin - intersection.point).normalize();
        let dot = ray
            .direction
            .dot(&light_direction.reflect(&intersection.normal));

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
