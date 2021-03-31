use std::path::{Path, PathBuf};
use std::sync::Arc;

use slog::{error, info};

use crate::resources::{DefaultLoader, ResourceLoader};

// Create or load a project under a directory
pub struct Project {
    // access the engine level logger
    logger: Arc<slog::Logger>,
    // project root directory
    base_dir: Option<PathBuf>,
    // load resource from the directory
    pub loader: Option<Box<dyn ResourceLoader>>,
}
impl Project {
    pub fn new(core_logger: Arc<slog::Logger>) -> Self {
        Self {
            logger: core_logger,
            base_dir: None,
            loader: None,
        }
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
    pub fn create_or_load(&mut self) -> &mut Self {
        let root = self.absolute_path().unwrap();
        if !root.exists() {
            // create project directory
            match std::fs::create_dir(&root) {
                Ok(_) => {
                    info!(self.logger, "project created at: {:?}", root);
                }
                Err(_) => {
                    error!(self.logger, "project creation error: {:?}", root);
                    return self;
                }
            }
        } else {
            info!(self.logger, "project loaded from: {:?}", root);
        }
        // initialize loader with the root directory
        let loader = Box::new(DefaultLoader::new(root));
        self.loader = Some(loader);

        self
    }

    // change base directory
    pub fn base_dir<P: AsRef<Path>>(mut self, base_dir: P) -> Self {
        self.base_dir = Some(base_dir.as_ref().into());
        self
    }
}
