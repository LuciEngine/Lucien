use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::graphics::{PrimitiveType, Mesh, Material};

// Load resources
pub trait ResourceLoader {
    fn load_text(&self, name: &str) -> Result<String>;
    fn load_bytes(&self, name: &str) -> Result<Vec<u8>>;
    // .off simply contains a mesh
    fn load_off(&self, name: &str) -> Result<Mesh>;
    // .obj can contain multiple models
    fn load_obj(&self, name: &str) -> Result<(Vec<Mesh>, Vec<Material>)>;
}

// Load from a base directory
pub struct DefaultLoader {
    base_dir: PathBuf,
}

impl DefaultLoader {
    pub fn new(root_dir: PathBuf) -> Self {
        DefaultLoader { base_dir: root_dir }
    }
}
impl ResourceLoader for DefaultLoader {
    fn load_text(&self, name: &str) -> Result<String> {
        let file_path = self.base_dir.join(name);
        let mut contents = String::new();
        File::open(&file_path)
            .with_context(|| format!("Failed to open file {:?}", &file_path))?
            .read_to_string(&mut contents)
            .with_context(|| format!("Failed to read file {:?}", &file_path))?;

        Ok(contents)
    }

    fn load_bytes(&self, name: &str) -> Result<Vec<u8>> {
        let file_path = self.base_dir.join(name);
        let mut contents = Vec::new();
        File::open(&file_path)
            .with_context(|| format!("Failed to open file {:?}", &file_path))?
            .read_to_end(&mut contents)
            .with_context(|| format!("Failed to read file {:?}", &file_path))?;

        Ok(contents)
    }

    fn load_off(&self, name: &str) -> Result<Mesh> {
        use glam::{vec3, Vec3};

        let file_path = self.base_dir.join(name);
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open file {:?}", &file_path))?;

        let reader = BufReader::new(file);
        let primitive_type = PrimitiveType::Triangle;
        let mut i = 0;
        let mut nv = 0;
        let mut mesh = Mesh::default();
        for line in reader.lines() {
            if i == 0 {
                i += 1;
                continue;
            }
            if i == 1 {
                let nums: Vec<u32> = line?.split(' ').flat_map(str::parse).collect();
                nv = nums[0];
                i += 1;
                continue;
            }
            if i > 1 && i <= 1 + nv {
                let nums: Vec<f32> = line?.split(' ').flat_map(str::parse).collect();
                mesh.positions.push(vec3(nums[0], nums[1], nums[2]));
                mesh.normals.push(Vec3::ZERO);
                i += 1;
                continue;
            }
            if i > 1 + nv {
                let id: Vec<usize> = line?.split(' ').flat_map(str::parse).collect();
                let v1 = mesh.positions[id[1]];
                let v2 = mesh.positions[id[2]];
                let v3 = mesh.positions[id[3]];
                let normal = (v2 - v1).cross(v3 - v1).normalize();
                mesh.normals[id[1]] += normal;
                mesh.normals[id[2]] += normal;
                mesh.normals[id[3]] += normal;
                match primitive_type {
                    PrimitiveType::Triangle => {
                        mesh.indices.push(id[1] as f32);
                        mesh.indices.push(id[2] as f32);
                        mesh.indices.push(id[3] as f32);
                    }
                    PrimitiveType::Line => {
                        mesh.indices.push(id[1] as f32);
                        mesh.indices.push(id[2] as f32);
                        mesh.indices.push(id[2] as f32);
                        mesh.indices.push(id[3] as f32);
                        mesh.indices.push(id[3] as f32);
                        mesh.indices.push(id[1] as f32);
                    }
                }
                i += 1;
                continue;
            }
        }
        for v in mesh.normals.iter_mut() {
            *v = v.normalize();
        }

        Ok(mesh)
    }

    fn load_obj(&self, name: &str) -> Result<(Vec<Mesh>, Vec<Material>)> {
        let file_path = self.base_dir.join(name);
        let (objs, mats) = tobj::load_obj(file_path, false)?;

        let mut meshes = vec![];
        let mut materials = vec![];
        for i in 0..objs.len() {
            meshes.push(Mesh::from_obj(&objs[i].mesh));
            materials.push(Material::from_obj(&mats[i]));
        }

        Ok((meshes, materials))
    }
}
