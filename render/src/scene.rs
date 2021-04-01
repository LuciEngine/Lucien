use crate::{AmbientLight, Camera, Material, Model, PointLight};
use anyhow::Result;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub light: PointLight, // todo: multiple lights
    pub ambient_light: AmbientLight,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
}

impl Scene {
    pub fn new(device: &wgpu::Device) -> Self {
        let models = vec![];
        let materials = vec![];
        let camera = Camera::default();
        let light = PointLight::default(device);
        let ambient_light = AmbientLight::default();

        Self {
            camera,
            light,
            ambient_light,
            models,
            materials,
        }
    }

    pub fn load(mut self, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self> {
        let (obj_models, obj_materials) = tobj::load_obj(path, true)?;
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
        if n_models < n_materials {
            for _ in 0..(n_models - n_materials) {
                self.materials.push(Material::default(device, queue)?);
            }
        }
        assert!(
            obj_models.len() == obj_materials.len(),
            "Models and materials count not equal!"
        );

        Ok(self)
    }
}
