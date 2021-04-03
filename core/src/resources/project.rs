use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use slog::{error, info};

use crate::logger::logger;
use crate::resources::loader;

// Create or load a project under a directory
pub struct Project {
    // project root directory
    base_dir: Option<PathBuf>,
}
impl Project {
    pub fn new() -> Self {
        Self { base_dir: None }
    }

    pub fn path(&self, name: &str) -> Option<PathBuf> {
        let res = self.base_dir.as_ref().unwrap().join(name).canonicalize();
        match res {
            Ok(_) => Some(res.unwrap()),
            Err(_) => None,
        }
    }

    fn absolute_path(&self) -> Option<PathBuf> {
        let relative = Path::new(self.base_dir.as_ref().unwrap());
        if relative.starts_with("~") {
            return dirs::home_dir().map(|mut abs| {
                abs.push(relative.strip_prefix("~").unwrap());
                abs
            });
        }
        let abs = std::env::current_dir().unwrap();
        let res = abs.join(relative).canonicalize();
        match res {
            Ok(_) => Some(res.unwrap()),
            Err(_) => None,
        }
    }

    // create project or load from existing
    pub fn create_or_load(&mut self) -> Result<()> {
        let root = self.absolute_path().unwrap();
        if !root.exists() {
            // create project directory
            match std::fs::create_dir(&root) {
                Ok(_) => {
                    info!(logger(), "project created at: {:?}", root);
                }
                Err(_) => {
                    error!(logger(), "project creation error: {:?}", root);
                    return Err(anyhow!("failed to create project"));
                }
            }
        } else {
            info!(logger(), "project loaded from: {:?}", root);
        }
        // initialize loader with the root directory
        loader::init_loader(root).context("Failed to init resource loader")?;

        Ok(())
    }

    // change base directory
    pub fn base_dir<P: AsRef<Path>>(mut self, base_dir: P) -> Self {
        self.base_dir = Some(base_dir.as_ref().into());
        self
    }
}
