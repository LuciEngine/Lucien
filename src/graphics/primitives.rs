use anyhow::Result;
use glam::{Vec2, Vec3};
use serde::Deserialize;

use crate::graphics::VertexAttributes;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PrimitiveType {
    Triangle,
    Line,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum RenderType {
    Png,
    Gif,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Light {
    pub position: Vec3,
    pub intensity: Vec3,
}

#[derive(Clone, Default, Deserialize)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,       // 2D texture coordinates of vertices.
    pub indices: Vec<f32>,          // Indices for vertices of each triangle.
    pub num_face_indices: Vec<u32>, // The number of vertices used by each face.
    pub material_id: Option<usize>, // Optional associated mesh id.
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Material {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub shininess: f32,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Camera {
    pub is_perspective: bool,
    pub position: Vec3,
    pub field_of_view: f32,
    pub aspect_ratio: f32,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Transform {
    pub angle: f32,
    pub distance: f32,
}

// Provide conversion between engine defined primitives
impl Mesh {
    pub fn from_obj(obj: &tobj::Mesh) -> Self {
        use glam::{vec2, vec3};

        let mut vertices = vec![];
        let mut norms = vec![];
        for i in 0..(obj.positions.len() / 3) {
            let v = &obj.positions;
            let n = &obj.normals;
            vertices.push(vec3(v[i], v[i + 1], v[i + 2]));
            norms.push(vec3(n[i], n[i + 1], n[i + 2]));
        }
        let mut coords = vec![];
        for i in 0..(obj.texcoords.len() / 2) {
            let t1 = obj.texcoords[i];
            let t2 = obj.texcoords[i + 1];
            coords.push(vec2(t1, t2));
        }
        let inds = obj.indices.iter().map(|&i| i as f32).collect();

        Self {
            positions: vertices,
            normals: norms,
            texcoords: coords,
            indices: inds,
            num_face_indices: obj.num_face_indices.clone(),
            material_id: obj.material_id,
        }
    }

    pub fn as_vertex_attrs(&self) -> Result<Vec<VertexAttributes>> {
        let mut vertices: Vec<VertexAttributes> = vec![];
        for i in 0..self.positions.len() {
            vertices.push(VertexAttributes::new(self.positions[i], self.normals[i]));
        }
        Ok(vertices)
    }
}

impl Material {
    pub fn from_obj(obj: &tobj::Material) -> Self {
        Self {
            ambient_color: Vec3::from(obj.ambient),
            diffuse_color: Vec3::from(obj.diffuse),
            specular_color: Vec3::from(obj.specular),
            shininess: obj.shininess,
        }
    }
}
