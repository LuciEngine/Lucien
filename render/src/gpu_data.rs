pub use crevice::std140::{AsStd140, Std140};

// We need the raw data to handle gpu memory layout,
// the problem is, if we just cast to [u8], gpu doesn't like that,
// it needs a padding. So we could utilize a library.

use glam::Vec3;
use mint::Vector3;

#[derive(AsStd140)]
pub struct PointLightRaw {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub intensity: f32,
}

#[derive(AsStd140)]
pub struct AmbientLightRaw {
    pub color: Vector3<f32>,
    pub intensity: f32,
}

#[derive(AsStd140)]
pub struct MaterialRaw {
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    shininess: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniformsRaw {
    pub view_proj: [[f32; 4]; 4],
    pub cam_pos: [f32; 3],
    _p0: f32,
    pub cam_dir: [f32; 3],
    _p1: f32,
    pub ambient_light_color: [f32; 3],
    _p2: f32,
    pub ambient_light_intensity: f32,
    _p3: [f32; 3],
}

impl PointLightRaw {
    pub fn from_vec3(position: &Vec3, color: &Vec3, intensity: f32) -> Self {
        Self {
            position: vec3_to_raw(position),
            color: vec3_to_raw(color),
            intensity,
        }
    }
}

#[allow(dead_code)]
impl AmbientLightRaw {
    pub fn from_vec3(color: &Vec3, intensity: f32) -> Self {
        Self {
            color: vec3_to_raw(color),
            intensity,
        }
    }
}

impl MaterialRaw {
    pub fn from_tobj(material: &tobj::Material) -> Self {
        Self {
            ambient: Vector3::from_slice(&material.ambient),
            diffuse: Vector3::from_slice(&material.diffuse),
            specular: Vector3::from_slice(&material.specular),
            shininess: material.shininess,
        }
    }
}

impl Default for MaterialRaw {
    fn default() -> Self {
        Self {
            ambient: vector3_one(),
            diffuse: vector3_one(),
            specular: vector3_half(),
            shininess: 0.0,
        }
    }
}

unsafe impl bytemuck::Pod for UniformsRaw {}
unsafe impl bytemuck::Zeroable for UniformsRaw {}

impl UniformsRaw {
    pub fn from(scene: &super::Scene) -> Self {
        Self {
            view_proj: scene.camera.view_proj,
            cam_pos: scene.camera.eye.into(),
            _p0: 0.0,
            cam_dir: scene.camera.direction().into(),
            _p1: 0.0,
            ambient_light_color: scene.ambient_light.color.into(),
            _p2: 0.0,
            ambient_light_intensity: scene.ambient_light.intensity,
            _p3: [0.0, 0.0, 0.0],
        }
    }
}

pub fn vec3_to_raw(v: &Vec3) -> Vector3<f32> {
    Vector3::from_slice(v.as_ref())
}

#[allow(dead_code)]
fn vector3_zero() -> Vector3<f32> {
    Vector3::from_slice(&[0.0, 0.0, 0.0])
}

#[allow(dead_code)]
fn vector3_half() -> Vector3<f32> {
    Vector3::from_slice(&[0.5, 0.5, 0.5])
}

#[allow(dead_code)]
fn vector3_one() -> Vector3<f32> {
    Vector3::from_slice(&[1.0, 1.0, 1.0])
}
