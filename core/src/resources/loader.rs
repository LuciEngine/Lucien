use anyhow::{anyhow, Context, Result};
use image::RgbaImage;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tobj::{Material, Model};

use crate::logger::logger;
use slog::debug;

static mut LOADER: Option<&dyn ResourceLoader> = None;
static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn set_singleton(loader: &'static dyn ResourceLoader) {
    set_singleton_inner(|| loader)
}

fn set_singleton_inner<F>(make_singleton: F)
where
    F: FnOnce() -> &'static dyn ResourceLoader,
{
    unsafe {
        if !std::ptr::read(&INITIALIZED).into_inner() {
            LOADER = Some(make_singleton());
            INITIALIZED.store(true, Ordering::Relaxed);

            debug!(logger(), "resource loader initialized.");
        }
    }
}

pub fn init_loader(root: PathBuf) -> Result<()> {
    let loader = Box::new(DefaultLoader::new(root));
    set_singleton(Box::leak(loader));

    Ok(())
}

pub fn loader() -> Result<&'static dyn ResourceLoader> {
    unsafe {
        if !std::ptr::read(&INITIALIZED).into_inner() {
            return Err(anyhow!("loader not intialized"));
        }
        Ok(LOADER.unwrap())
    }
}

// Load resources
pub trait ResourceLoader {
    fn load_text(&self, name: &str) -> Result<String>;
    fn load_bytes(&self, name: &str) -> Result<Vec<u8>>;
    // .obj can contain multiple models
    fn load_obj(&self, name: &str) -> Result<(Vec<Model>, Vec<Material>)>;
    fn load_rgba(&self, name: &str) -> Result<RgbaImage>;
}

// Load from a base directory
#[derive(Debug)]
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

    fn load_obj(&self, name: &str) -> Result<(Vec<Model>, Vec<Material>)> {
        let file_path = self.base_dir.join(name);
        let (objs, materials) = tobj::load_obj(&file_path, true)
            .with_context(|| format!("Failed to load obj: {:?}", &file_path))?;
        // let meshes = objs.into_iter().map(|obj| obj.mesh).collect();
        Ok((objs, materials))
    }

    fn load_rgba(&self, name: &str) -> Result<RgbaImage> {
        let file_path = self.base_dir.join(name);
        let img = image::open(&file_path)
            .with_context(|| format!("Failed to open file {:?}", &file_path))?;
        Ok(img.to_rgba8())
    }
}
