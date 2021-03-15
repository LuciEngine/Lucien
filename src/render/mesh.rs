use crate::render::Vertex;
use wgpu::util::DeviceExt;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub material: usize,
    pub num_indices: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, mesh: &tobj::Mesh, name: &str) -> Self {
        let vertices = Vertex::from_tobj(mesh);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} vertex buffer", name).as_str()),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} index buffer", name).as_str()),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = mesh.indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            material: mesh.material_id.unwrap_or(0),
            num_indices,
        }
    }
}

pub struct Model {
    pub mesh: Mesh,
    pub name: String,
}

impl Model {
    pub fn new(device: &wgpu::Device, model: &tobj::Model) -> Self {
        let name = model.name.as_str().to_string();
        let mesh = Mesh::new(device, &model.mesh, &name.as_str());
        Self { mesh, name }
    }
}
