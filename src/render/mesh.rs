use crate::render::Vertex;
use tobj;
use wgpu;
use wgpu::util::DeviceExt;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub material: usize,
    pub num_indices: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, mesh: &tobj::Mesh, name: &String) -> Self {
        let mut vertices: Vec<Vertex> = vec![];
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vertex {
                position: [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2]
                ],
                normal: [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2]
                ],
                tex_coord: [
                    mesh.texcoords[i * 2],
                    mesh.texcoords[i * 2 + 1]
                ],
            });
        }
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(format!("{} vertex buffer", name).as_str()),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
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
        let mesh = Mesh::new(device, &model.mesh, &name);
        Self { mesh, name }
    }
}
