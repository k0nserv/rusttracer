use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::rc::Rc;

use color::Color;
use config;
use config::Object;
use geometry::{Intersectable, Transformable};
use geometry::{Plane, Sphere};
use intersection::Intersection;
use light;
use material::Material;
use math::{Point3, Vector3};
use mesh_loader::MeshLoader;
use ray::Ray;

#[derive(Debug, Clone)]
pub struct SceneConfigLoadError {
    description: String,
}

impl SceneConfigLoadError {
    fn new(description: String) -> Self {
        SceneConfigLoadError { description }
    }
}

impl fmt::Display for SceneConfigLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load scene from config. {}", self.description)
    }
}

impl Error for SceneConfigLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Box<dyn light::Light>>,
    pub ambient_color: Color,
    pub clear_color: Color,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<dyn Intersectable>>,
        lights: Vec<Box<dyn light::Light>>,
        ambient_color: Color,
        clear_color: Color,
    ) -> Scene {
        Scene {
            objects,
            lights,
            ambient_color,
            clear_color,
        }
    }

    pub fn new_from_config(
        scene: &config::Scene,
        materials: &HashMap<String, Rc<Material>>,
        mesh_loader: &MeshLoader,
        fallback_material: Rc<Material>,
    ) -> Result<Scene, SceneConfigLoadError> {
        let mut objects: Vec<Box<dyn Intersectable>> = vec![];

        for object in &scene.objects {
            match *object {
                Object::Sphere {
                    radius,
                    ref transforms,
                    ref material_name,
                } => {
                    let material = match material_name {
                        None => fallback_material.clone(),
                        Some(name) => {
                            assert!(
                                materials.contains_key(name),
                                "Invalid material name: {}",
                                name
                            );
                            materials[name].clone()
                        }
                    };
                    let mut sphere = Box::new(Sphere::new(Point3::at_origin(), radius, material));
                    Self::apply_transforms(sphere.as_mut() as &mut dyn Transformable, transforms);
                    objects.push(sphere as Box<dyn Intersectable>);
                }
                Object::Plane {
                    normal,
                    ref transforms,
                    ref material_name,
                } => {
                    let material = match material_name {
                        None => fallback_material.clone(),
                        Some(name) => {
                            assert!(
                                materials.contains_key(name),
                                "Invalid material name: {}",
                                name
                            );
                            materials[name].clone()
                        }
                    };
                    let mut plane = Box::new(Plane::new(
                        Point3::at_origin(),
                        Vector3::from(normal),
                        material,
                    ));
                    Self::apply_transforms(plane.as_mut() as &mut dyn Transformable, transforms);
                    objects.push(plane as Box<dyn Intersectable>);
                }
                Object::Mesh {
                    ref path,
                    ref transforms,
                } => {
                    let mut meshes =
                        match mesh_loader.load(Path::new(&path), fallback_material.clone()) {
                            Ok(meshes) => meshes,
                            Err(error) => {
                                println!("Failed to load scene: {}", error);
                                return Err(SceneConfigLoadError::new(error.to_string()));
                            }
                        };

                    for mesh in &mut meshes {
                        Self::apply_transforms(mesh.as_mut() as &mut dyn Transformable, transforms);
                    }
                    let mut intersectables = meshes
                        .into_iter()
                        .map(|mesh| mesh as Box<dyn Intersectable>)
                        .collect();

                    objects.append(&mut intersectables);
                }
            }
        }

        let lights: Vec<Box<dyn light::Light>> = scene
            .lights
            .iter()
            .map(|light| match *light {
                config::Light::PointLight {
                    origin,
                    color,
                    intensity,
                    falloff,
                } => Box::new(light::Point::new(
                    Point3::from(origin),
                    Color::from(color),
                    intensity,
                    falloff.unwrap_or(light::Falloff::InverseSquare),
                )) as Box<dyn light::Light>,

                config::Light::DirectionalLight {
                    direction,
                    color,
                    intensity,
                } => Box::new(light::Directional::new(
                    Vector3::from(direction),
                    Color::from(color),
                    intensity,
                )) as Box<dyn light::Light>,
            })
            .collect();

        Ok(Self::new(
            objects,
            lights,
            Color::from(scene.ambient_color),
            Color::from(scene.clear_color),
        ))
    }

    fn apply_transforms(
        shape: &mut dyn Transformable,
        transforms: &Option<Vec<config::Transform>>,
    ) {
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
