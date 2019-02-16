use std::error;
use std::fmt;
use std::path::Path;

use color::Color;
use config;
use config::Object;
use geometry::{Intersectable, Transformable};
use geometry::{Plane, Sphere};
use intersection::Intersection;
use lights::PointLight;
use material::Material;
use math::{Point3, Vector3};
use mesh_loader::MeshLoader;
use ray::Ray;

#[derive(Debug, Clone)]
pub struct SceneConfigLoadError;

impl fmt::Display for SceneConfigLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load scene from config")
    }
}

impl error::Error for SceneConfigLoadError {
    fn description(&self) -> &str {
        "failed to load scene from config"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct Scene {
    pub objects: Vec<Box<Intersectable>>,
    pub lights: Vec<Box<PointLight>>,
    pub clear_color: Color,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<Intersectable>>,
        lights: Vec<Box<PointLight>>,
        clear_color: Color,
    ) -> Scene {
        Scene {
            objects,
            lights,
            clear_color,
        }
    }

    pub fn new_from_config(
        scene: &config::Scene,
        materials: &[Material],
        mesh_loader: &MeshLoader,
        fallback_material: Material,
    ) -> Result<Scene, SceneConfigLoadError> {
        let mut objects: Vec<Box<Intersectable>> = vec![];

        scene.objects.iter().for_each(|object| match *object {
            Object::Sphere {
                radius,
                ref transforms,
                material_id,
            } => {
                let material = match material_id {
                    None => fallback_material,
                    Some(id) => {
                        assert!(id < materials.len(), "Invalid material_id");
                        materials[id]
                    }
                };
                let mut sphere = Box::new(Sphere::new(Point3::at_origin(), radius, material));
                Self::apply_transforms(sphere.as_mut() as &mut Transformable, transforms);
                objects.push(sphere as Box<Intersectable>);
            }
            Object::Plane {
                normal,
                ref transforms,
                material_id,
            } => {
                let material = match material_id {
                    None => fallback_material,
                    Some(id) => {
                        assert!(id < materials.len(), "Invalid material_id");
                        materials[id]
                    }
                };
                let mut plane = Box::new(Plane::new(
                    Point3::at_origin(),
                    Vector3::new_from_slice(normal),
                    material,
                ));
                Self::apply_transforms(plane.as_mut() as &mut Transformable, transforms);
                objects.push(plane as Box<Intersectable>);
            }
            Object::Mesh {
                ref path,
                ref transforms,
            } => {
                let mut meshes = mesh_loader.load(Path::new(&path), &fallback_material);
                for mesh in &mut meshes {
                    Self::apply_transforms(mesh.as_mut() as &mut Transformable, transforms);
                }
                let mut intersectables = meshes
                    .into_iter()
                    .map(|mesh| mesh as Box<Intersectable>)
                    .collect();

                objects.append(&mut intersectables);
            }
        });

        let lights = scene
            .lights
            .iter()
            .map(|light| match *light {
                config::Light::PointLight {
                    origin,
                    color,
                    intensity,
                } => Box::new(PointLight::new(
                    Point3::new_from_slice(origin),
                    Color::new_from_slice(color),
                    intensity,
                )),
            })
            .collect();

        Ok(Self::new(
            objects,
            lights,
            Color::new_from_slice(scene.clear_color),
        ))
    }

    fn apply_transforms(shape: &mut Transformable, transforms: &Option<Vec<config::Transform>>) {
        if let Some(ref transforms_to_apply) = *transforms {
            for transform in transforms_to_apply {
                transform.perform(shape);
            }
        }
    }

    pub fn intersect(&self, ray: Ray, cull: bool) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;

        for shape in &self.objects {
            if let Some(intersection) = shape.intersect(ray, cull) {
                if let Some(closest) = closest_intersection {
                    if intersection.t < closest.t {
                        closest_intersection = Some(intersection)
                    }
                } else {
                    closest_intersection = Some(intersection)
                }
            }
        }

        closest_intersection
    }

    pub fn first_intersection(&self, ray: Ray, cull: bool, distance: f32) -> Option<Intersection> {
        for object in &self.objects {
            if let Some(hit) = object.intersect(ray, cull) {
                if hit.t < distance {
                    return Some(hit);
                }
            }
        }

        None
    }
}
