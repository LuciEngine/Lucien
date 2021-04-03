use crate::{AmbientLight, Camera, Material, Model, PointLight};
use anyhow::Result;

use lucien_core::logger::logger;
use lucien_core::resources::loader;
use slog::warn;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub light: PointLight, // todo: multiple lights
    pub ambient_light: AmbientLight,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    logger: Logger,
}

impl Scene {
    pub fn new(device: &wgpu::Device) -> Result<Self> {
        let models = vec![];
        let materials = vec![];
        let camera = Camera::default();
        let light = PointLight::default(device);
        let ambient_light = AmbientLight::default();
        let logger = CoreLogBuilder::new().get_logger();

        Ok(Self {
            camera,
            light,
            ambient_light,
            models,
            materials,
        })
    }

    pub fn load(mut self, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self> {
        let (obj_models, obj_materials) = loader()?.load_obj(path)?;
        obj_models.iter().for_each(|model| {
            self.models.push(Model::new(device, model));
        });
        obj_materials.iter().for_each(|material| {
            self.materials
                .push(Material::new(device, queue, material).unwrap());
        });
        // if material is missing from the file, use default
        let n_models = obj_models.len();
        let n_materials = obj_materials.len();
        if n_models > n_materials {
            let msg = format!("material missing from file {}, use default", path);
            warn!(logger(), "{}", msg);
            for _ in 0..(n_models - n_materials) {
                self.materials.push(Material::default(device, queue)?);
            }
        }
        assert!(
            self.materials.len() == self.models.len(),
            "Models and materials count not equal!"
        );

        Ok(self)
    }
}
