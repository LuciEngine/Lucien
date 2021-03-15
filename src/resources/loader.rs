use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use tobj::{ Mesh, Material };

// Load resources
pub trait ResourceLoader {
    fn load_text(&self, name: &str) -> Result<String>;
    fn load_bytes(&self, name: &str) -> Result<Vec<u8>>;
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

    fn load_obj(&self, name: &str) -> Result<(Vec<Mesh>, Vec<Material>)> {
        let file_path = self.base_dir.join(name);
        let (objs, materials) = tobj::load_obj(&file_path, false)
            .with_context(|| format!("Failed to load obj: {:?}", &file_path))?;
        let meshes = objs.into_iter().map(|obj| { obj.mesh }).collect();
        Ok((meshes, materials))
    }
}
