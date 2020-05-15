extern crate tobj;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use color::Color;
use geometry::triangle::Normal;
use geometry::{ExtentVolume, Mesh, Octree, SimpleTriangleStorage, Triangle, AABB};
use material::{IllumninationModel, IllumninationModelParsingError, Material, OptionalTexture};
use math::{Point3, Vector3};
use texture;

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

pub struct MeshLoader {
    root_path: PathBuf,
}

impl MeshLoader {
    pub fn new(root_path: PathBuf) -> MeshLoader {
        MeshLoader { root_path }
    }

    pub fn load(
        &self,
        path: &Path,
        fallback_material: Rc<Material>,
    ) -> Result<Vec<Box<Mesh<ExtentVolume, Octree>>>, MeshLoadError> {
        let final_path = self.root_path.join(path);
        let result = tobj::load_obj(&final_path);
        if let Err(ref error) = result {
            println!("Load error: {}", error);
        }
        assert!(result.is_ok());

        let (models, materials) = result.unwrap();
        let mut meshes = vec![];
        let mut material_cache = HashMap::new();

        for (i, m) in materials.iter().enumerate() {
            let illumination_model = match m.illumination_model {
                Some(model) => IllumninationModel::try_from(model)?,
                None => IllumninationModel::DiffuseSpecular,
            };

            let ambient_texture =
                self.load_texture_from_file(final_path.as_path(), &m.ambient_texture)?;
            let diffuse_texture =
                self.load_texture_from_file(final_path.as_path(), &m.diffuse_texture)?;
            let specular_texture =
                self.load_texture_from_file(final_path.as_path(), &m.specular_texture)?;

            let mat = Rc::new(Material::new_with_textures(
                Color::new_f32(m.ambient[0], m.ambient[1], m.ambient[2]),
                ambient_texture,
                Color::new_f32(m.diffuse[0], m.diffuse[1], m.diffuse[2]),
                diffuse_texture,
                Color::new_f32(m.specular[0], m.specular[1], m.specular[2]),
                specular_texture,
                m.shininess,
                illumination_model,
                Some(1000.0 / m.shininess),
                Some(m.optical_density),
            ));

            material_cache.insert(i, mat);
        }

        for (i, m) in models.iter().enumerate() {
            let mut triangles = vec![];
            let mesh = &m.mesh;
            if mesh.indices.is_empty() && mesh.positions.is_empty() {
                continue;
            }
            println!("Mesh with index {}", i);
            println!("Mesh name {}", m.name);
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
                        texture::TextureCoord::new(
                            mesh.texcoords[i0 * 2],
                            mesh.texcoords[i0 * 2 + 1],
                        ),
                        texture::TextureCoord::new(
                            mesh.texcoords[i1 * 2],
                            mesh.texcoords[i1 * 2 + 1],
                        ),
                        texture::TextureCoord::new(
                            mesh.texcoords[i2 * 2],
                            mesh.texcoords[i2 * 2 + 1],
                        ),
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

            let mesh = Box::new(Mesh::new(triangles));
            meshes.push(mesh);
        }

        Ok(meshes)
    }

    fn load_texture_from_file(
        &self,
        obj_path: &Path,
        texture: &str,
    ) -> Result<OptionalTexture, MeshLoadError> {
        if texture.is_empty() {
            return Ok(None);
        }

        let full_path = if let Some(resolve_path) = obj_path.parent() {
            resolve_path.join(texture)
        } else {
            PathBuf::from(texture)
        };
        let texture = texture::file::File::new(full_path)?;

        Ok(Some(Rc::new(texture)))
    }
}
