extern crate tobj;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

use geometry::{Mesh, Triangle, AABB};
use geometry::triangle::Normal;
use material::{IllumninationModel, Material};
use math::{Point3, Vector3};
use color::Color;


pub struct MeshLoader {
    root_path: PathBuf,
}

impl MeshLoader {
    pub fn new(root_path: PathBuf) -> MeshLoader {
        MeshLoader {
            root_path: root_path,
        }
    }

    pub fn load(&self, path: &Path, fallback_material: &Material) -> Vec<Box<Mesh<AABB>>> {
        let final_path = self.root_path.join(path);
        let result = tobj::load_obj(&final_path);
        if let &Err(ref error) = &result {
            println!("Load error: {}", error);
        }
        assert!(result.is_ok());

        let (models, materials) = result.unwrap();
        let mut meshes = vec![];
        let mut material_cache = HashMap::new();

        for (i, m) in materials.iter().enumerate() {
            let illumination_model = match m.illumination_model {
                Some(model) => IllumninationModel::from(model),
                None => IllumninationModel::DiffuseSpecular,
            };

            let mat = Box::new(Material::new(
                Color::new_f32(m.ambient[0], m.ambient[1], m.ambient[2]),
                Color::new_f32(m.diffuse[0], m.diffuse[1], m.diffuse[2]),
                Color::new_f32(m.specular[0], m.specular[1], m.specular[2]),
                m.shininess,
                illumination_model,
                None,
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
            let use_vertex_normals = mesh.normals.len() > 0;

            if use_vertex_normals {
                println!("Using vertex normals");
            }


            for f in 0..mesh.indices.len() / 3 {
                let i0 = mesh.indices[f * 3 + 0] as usize;
                let i1 = mesh.indices[f * 3 + 1] as usize;
                let i2 = mesh.indices[f * 3 + 2] as usize;


                let p0 = Point3::new(
                    mesh.positions[i0 * 3 + 0],
                    mesh.positions[i0 * 3 + 1],
                    mesh.positions[i0 * 3 + 2],
                );
                let p1 = Point3::new(
                    mesh.positions[i1 * 3 + 0],
                    mesh.positions[i1 * 3 + 1],
                    mesh.positions[i1 * 3 + 2],
                );
                let p2 = Point3::new(
                    mesh.positions[i2 * 3 + 0],
                    mesh.positions[i2 * 3 + 1],
                    mesh.positions[i2 * 3 + 2],
                );

                let normal;
                if use_vertex_normals {
                    let n0 = Vector3::new(
                        mesh.normals[i0 * 3 + 0],
                        mesh.normals[i0 * 3 + 1],
                        mesh.normals[i0 * 3 + 2],
                    );
                    let n1 = Vector3::new(
                        mesh.normals[i1 * 3 + 0],
                        mesh.normals[i1 * 3 + 1],
                        mesh.normals[i1 * 3 + 2],
                    );
                    let n2 = Vector3::new(
                        mesh.normals[i2 * 3 + 0],
                        mesh.normals[i2 * 3 + 1],
                        mesh.normals[i2 * 3 + 2],
                    );

                    normal = Some(Normal::Vertex(n0, n1, n2));
                } else {
                    let ab = p0 - p1;
                    let ac = p0 - p2;

                    normal = Some(Normal::Face(ab.cross(&ac).normalize()));
                }


                let mut material = fallback_material;
                if let Some(id) = mesh.material_id {
                    if let Some(m) = material_cache.get(&id) {
                        material = m;
                    }
                }

                triangles.push(Box::new(Triangle::new(
                    p0,
                    p1,
                    p2,
                    normal.unwrap(),
                    *material,
                )));
            }

            let mesh = Box::new(Mesh::new(triangles));
            meshes.push(mesh);
        }

        meshes
    }
}
