use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::camera;
use crate::color::Color;
use crate::config;
use crate::material;
use crate::mesh_loader::MeshLoader;
use crate::renderer;
use crate::scene;
use crate::texture;

pub struct ConfigLoader {
    fallback_material: Rc<material::Material>,
    named_textures: HashMap<String, Rc<dyn texture::Texture>>,
}

impl ConfigLoader {
    pub fn new(fallback_material: Rc<material::Material>) -> Self {
        Self {
            fallback_material,
            named_textures: HashMap::default(),
        }
    }

    pub fn register_named_texture(&mut self, name: &str, texture: Rc<dyn texture::Texture>) {
        self.named_textures.insert(name.to_string(), texture);
    }

    pub fn load_renderer_from_config(
        &self,
        path: &str,
    ) -> Result<(renderer::Renderer, config::Config), Box<dyn std::error::Error>> {
        let parsed_config = config::Config::new_from_file(path)?;
        let scene_path = Path::new(path).parent().unwrap();
        let mut mesh_loader = MeshLoader::new(scene_path.to_path_buf());

        let materials = parsed_config
            .materials
            .iter()
            .map(|material_config| {
                // TODO: Error handling
                (
                    material_config.name.to_owned(),
                    Rc::new(material::Material::new_with_textures(
                        Color::from(material_config.ambient_color),
                        self.resolve_texture(&material_config.ambient_texture)
                            .unwrap(),
                        Color::from(material_config.diffuse_color),
                        self.resolve_texture(&material_config.diffuse_texture)
                            .unwrap(),
                        Color::from(material_config.specular_color),
                        self.resolve_texture(&material_config.specular_texture)
                            .unwrap(),
                        material_config.specular_exponent,
                        material_config.illumination_model,
                        material_config.reflection_coefficient,
                        material_config.refraction_coefficient,
                    )),
                )
            })
            .collect();

        let scene = scene::Scene::new_from_config(
            parsed_config.scenes.first().unwrap(),
            &materials,
            &mut mesh_loader,
            Rc::clone(&self.fallback_material),
        )?;
        let camera_config = parsed_config
            .cameras
            .first()
            .expect("Config should contain at least one valid camera");
        let camera = camera::Camera::from(camera_config);
        let renderer = renderer::Renderer::new(scene, camera, parsed_config.super_sampling);

        Ok((renderer, parsed_config))
    }

    fn resolve_texture(
        &self,
        texture: &Option<config::Texture>,
    ) -> Result<material::OptionalTexture, Box<dyn std::error::Error>> {
        match texture {
            None => Ok(None),
            Some(texture) => match texture {
                config::Texture::Named(name) => {
                    let tex = self.named_textures.get(name).unwrap();

                    Ok(Some(Rc::clone(tex)))
                }
            },
        }
    }
}
