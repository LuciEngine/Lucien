use crate::render::*;

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

    pub fn load(mut self, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let (obj_models, obj_materials) = tobj::load_obj(path, true).unwrap();
        obj_models.iter().for_each(|model| {
            self.models.push(Model::new(device, model));
        });
        obj_materials.iter().for_each(|material| {
            self.materials
                .push(Material::new(device, queue, material).unwrap());
        });
        self
    }
}
