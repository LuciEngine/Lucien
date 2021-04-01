use bytemuck::{Pod, Zeroable};
use glam::Vec3;

use lucien_core::logger::CoreLogBuilder;
use slog::warn;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

#[allow(dead_code)]
fn compute_normal(v0: &[f32; 3], v1: &[f32; 3], v2: &[f32; 3]) -> [f32; 3] {
    let n0 = Vec3::from(*v0);
    let n1 = Vec3::from(*v1);
    let n2 = Vec3::from(*v2);
    let f0 = n0 - n2;
    let f2 = n1 - n2;
    f0.cross(f2).normalize().into()
}

impl Vertex {
    pub fn from_tobj(mesh: &tobj::Mesh) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = vec![];
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Self {
                position: [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ],
                normal: if !mesh.normals.is_empty() {
                    [
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                },
                tex_coord: if !mesh.texcoords.is_empty() {
                    [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                } else {
                    [0.0, 0.0]
                },
            });
        }
        let logger = CoreLogBuilder::new().get_logger();

        // todo calculate normals after I figured out what is the correct face...
        if mesh.texcoords.is_empty() {
            warn!(logger, "texture coord missing for mesh");
        }
        if mesh.normals.is_empty() {
            warn!(logger, "normals missing for mesh");
        }
        vertices
    }

    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}
