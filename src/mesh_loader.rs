extern crate tobj;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::color::Color;
use crate::geometry::triangle::Normal;
use crate::geometry::{BoundingVolume, Instance, Mesh, Triangle, TriangleStorage};
use crate::material::{
    IllumninationModel, IllumninationModelParsingError, Material, OptionalTexture,
};
use crate::math::{Point3, Vector3};
use crate::texture;

#[derive(Debug)]
pub enum MeshLoadError {
    TextureLoadError(texture::file::FileError),
    IllumenationModelParsingError(IllumninationModelParsingError),
}

impl From<texture::file::FileError> for MeshLoadError {
    fn from(texture_error: texture::file::FileError) -> Self {
        MeshLoadError::TextureLoadError(texture_error)
    }
}

impl From<IllumninationModelParsingError> for MeshLoadError {
    fn from(illumination_model_error: IllumninationModelParsingError) -> Self {
        MeshLoadError::IllumenationModelParsingError(illumination_model_error)
    }
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeshLoadError::TextureLoadError(inner_error) => write!(
                f,
                "Failed to load meshes with texture load error: {}",
                inner_error
            ),
            MeshLoadError::IllumenationModelParsingError(inner_error) => write!(
                f,
                "Failed to load meshes with illumination model error: {}",
                inner_error
            ),
        }
    }
}

impl Error for MeshLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MeshLoadError::TextureLoadError(inner_error) => Some(inner_error),
            MeshLoadError::IllumenationModelParsingError(inner_error) => Some(inner_error),
        }
    }
}

type LoadedObj = Rc<(Vec<tobj::Model>, Vec<tobj::Material>)>;

pub struct MeshLoader<V, S> {
    root_path: PathBuf,
    mesh_cache: HashMap<String, Rc<Mesh<V, S>>>,
    obj_cache: HashMap<String, LoadedObj>,
}

impl<'a, V: BoundingVolume, S: 'a + TriangleStorage<'a>> MeshLoader<V, S> {
    pub fn new(root_path: PathBuf) -> MeshLoader<V, S> {
        MeshLoader {
            root_path,
            mesh_cache: HashMap::default(),
            obj_cache: HashMap::default(),
        }
    }

    pub fn load(
        &mut self,
        path: &Path,
        fallback_material: Rc<Material>,
    ) -> Result<Vec<Box<Mesh<V, S>>>, MeshLoadError> {
        let final_path = self.root_path.join(path);
        let obj = self.load_obj(&final_path);
        let (models, materials) = obj.as_ref();
        let mut meshes = vec![];
        let material_cache = self.build_material_cache(&final_path, materials)?;

        for m in models.iter() {
            if let Some(mesh) = self.prepare_mesh(m, &material_cache, &fallback_material) {
                meshes.push(Box::new(mesh));
            }
        }

        Ok(meshes)
    }

    pub fn load_instance(
        &mut self,
        path: &Path,
        fallback_material: Rc<Material>,
    ) -> Result<Vec<Box<Instance<V, S>>>, MeshLoadError> {
        let final_path = self.root_path.join(path);
        let obj = self.load_obj(&final_path);
        let (models, materials) = obj.as_ref();
        let mut meshes = vec![];
        let material_cache = self.build_material_cache(&final_path, materials)?;

        for m in models.iter() {
            let cache_key = Self::build_cache_key(&final_path.to_string_lossy(), &m.name);

            if let Some(mesh) = self.mesh_cache.get(&cache_key) {
                meshes.push(Box::new(Instance::new(
                    Rc::clone(mesh),
                    fallback_material.clone(),
                )));
                continue;
            }

            if let Some(mut mesh) = self.prepare_mesh(m, &material_cache, &fallback_material) {
                // Make sure any acceleration structures are built
                // in model space.
                mesh.rebuild_accelleration_structure();

                let mesh = Rc::new(mesh);
                let instance = Box::new(Instance::new(Rc::clone(&mesh), fallback_material.clone()));
                self.mesh_cache
                    .insert(cache_key.to_owned(), Rc::clone(&mesh));

                meshes.push(instance);
            }
        }

        Ok(meshes)
    }

    fn prepare_mesh(
        &self,
        model: &tobj::Model,
        material_cache: &HashMap<usize, Rc<Material>>,
        fallback_material: &Rc<Material>,
    ) -> Option<Mesh<V, S>> {
        let mesh = &model.mesh;
        if mesh.indices.is_empty() && mesh.positions.is_empty() {
            return None;
        }

        let mut triangles = Vec::with_capacity(mesh.indices.len() / 3);
        println!("Mesh name {}", model.name);
        println!("Num indices: {}", mesh.indices.len());
        println!("Num vertices: {}", mesh.positions.len());
        println!("Num normals: {}", mesh.normals.len());
        println!("Num texture coords: {}", mesh.texcoords.len());
        let use_vertex_normals = !mesh.normals.is_empty();
        let has_texture_coords = !mesh.texcoords.is_empty();

        if use_vertex_normals {
            println!("Using vertex normals");
        }

        if has_texture_coords {
            println!("Using textures");
        }

        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[f * 3] as usize;
            let i1 = mesh.indices[f * 3 + 1] as usize;
            let i2 = mesh.indices[f * 3 + 2] as usize;

            let p0 = Point3::new(
                mesh.positions[i0 * 3],
                mesh.positions[i0 * 3 + 1],
                mesh.positions[i0 * 3 + 2],
            );
            let p1 = Point3::new(
                mesh.positions[i1 * 3],
                mesh.positions[i1 * 3 + 1],
                mesh.positions[i1 * 3 + 2],
            );
            let p2 = Point3::new(
                mesh.positions[i2 * 3],
                mesh.positions[i2 * 3 + 1],
                mesh.positions[i2 * 3 + 2],
            );

            let normal = if use_vertex_normals {
                let n0 = Vector3::new(
                    mesh.normals[i0 * 3],
                    mesh.normals[i0 * 3 + 1],
                    mesh.normals[i0 * 3 + 2],
                );
                let n1 = Vector3::new(
                    mesh.normals[i1 * 3],
                    mesh.normals[i1 * 3 + 1],
                    mesh.normals[i1 * 3 + 2],
                );
                let n2 = Vector3::new(
                    mesh.normals[i2 * 3],
                    mesh.normals[i2 * 3 + 1],
                    mesh.normals[i2 * 3 + 2],
                );

                Normal::Vertex(n0, n1, n2)
            } else {
                let ab = p0 - p1;
                let ac = p0 - p2;

                Normal::Face(ab.cross(&ac).normalize())
            };

            let texture_coords = if has_texture_coords {
                Some([
                    texture::TextureCoord::new(mesh.texcoords[i0 * 2], mesh.texcoords[i0 * 2 + 1]),
                    texture::TextureCoord::new(mesh.texcoords[i1 * 2], mesh.texcoords[i1 * 2 + 1]),
                    texture::TextureCoord::new(mesh.texcoords[i2 * 2], mesh.texcoords[i2 * 2 + 1]),
                ])
            } else {
                None
            };

            let mut material = fallback_material.clone();
            if let Some(id) = mesh.material_id {
                if let Some(m) = material_cache.get(&id) {
                    material = m.clone();
                }
            }

            triangles.push(Triangle::new(p0, p1, p2, normal, texture_coords, material));
        }

        Some(Mesh::new(triangles))
    }

    fn load_obj(&mut self, path: &Path) -> LoadedObj {
        Rc::clone(
            self.obj_cache
                .entry(path.to_string_lossy().to_string())
                .or_insert_with(|| {
                    let result = tobj::load_obj(
                        path,
                        &tobj::LoadOptions {
                            single_index: false,
                            triangulate: true,
                            ignore_points: true,
                            ignore_lines: true,
                        },
                    );
                    // TODO: Better error handling
                    let (models, materials) = result.unwrap_or_else(|e| {
                        panic!(
                            "Failed to load data from {} with error: {}",
                            path.display(),
                            e
                        )
                    });
                    let materials = materials.unwrap_or_else(|e| {
                        println!("Failed to load materials for {:?} with error: {}", path, e);

                        Vec::default()
                    });

                    Rc::new((models, materials))
                }),
        )
    }

    fn build_material_cache(
        &self,
        path: &Path,
        materials: &[tobj::Material],
    ) -> Result<HashMap<usize, Rc<Material>>, MeshLoadError> {
        let mut material_cache = HashMap::new();
        for (i, m) in materials.iter().enumerate() {
            let illumination_model = match m.illumination_model {
                Some(model) => IllumninationModel::try_from(model)?,
                None => IllumninationModel::DiffuseSpecular,
            };

            let ambient_texture =
                self.load_texture_from_file(path, m.ambient_texture.as_deref())?;
            let diffuse_texture =
                self.load_texture_from_file(path, m.diffuse_texture.as_deref())?;
            let specular_texture =
                self.load_texture_from_file(path, m.specular_texture.as_deref())?;

            let ambient = m.ambient.unwrap_or([0.0; 3]);
            let diffuse = m.diffuse.unwrap_or([0.0; 3]);
            let specular = m.specular.unwrap_or([0.0; 3]);

            let mat = Rc::new(Material::new_with_textures(
                Color::new_f32(ambient[0], ambient[1], ambient[2]),
                ambient_texture,
                Color::new_f32(diffuse[0], diffuse[1], diffuse[2]),
                diffuse_texture,
                Color::new_f32(specular[0], specular[1], specular[2]),
                specular_texture,
                m.shininess.unwrap_or(0.0),
                illumination_model,
                m.shininess.map(|s| 1000.0 / s),
                m.optical_density,
            ));

            material_cache.insert(i, mat);
        }

        Ok(material_cache)
    }

    fn load_texture_from_file(
        &self,
        obj_path: &Path,
        texture: Option<&str>,
    ) -> Result<OptionalTexture, MeshLoadError> {
        let Some(path) = texture else { return Ok(None) };
        if path.is_empty() {
            return Ok(None);
        }

        let full_path = if let Some(resolve_path) = obj_path.parent() {
            resolve_path.join(path)
        } else {
            PathBuf::from(path)
        };
        let texture = texture::file::File::new(full_path)?;

        Ok(Some(Rc::new(texture)))
    }

    fn build_cache_key(filename: &str, mesh_name: &str) -> String {
        format!("{}-{}", filename, mesh_name)
    }
}
