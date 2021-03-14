pub(super) use crevice::std140::{AsStd140, Std140};

// We need the raw data to handle gpu memory layout,
// the problem is, if we just cast to [u8], gpu doesn't like that,
// it needs a padding. So we could utilize a library.

use glam::Vec3;
use mint::Vector3;

#[derive(AsStd140)]
pub struct LightRaw {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl LightRaw {
    pub fn from_vec3(position: &Vec3, color: &Vec3) -> Self {
        Self {
            position: Vector3::from_slice(position.as_ref()),
            color: Vector3::from_slice(color.as_ref()),
        }
    }
}

#[derive(AsStd140)]
pub struct MaterialRaw {
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    shininess: f32,
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
