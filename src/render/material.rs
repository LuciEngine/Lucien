use crate::render::Texture;

use anyhow::Result;
use crevice::std140::{AsStd140, Std140};
use mint::Vector3;
use tobj;
use wgpu;
use wgpu::util::DeviceExt;

pub struct Material {
    pub diffuse_texture: Texture,
    pub name: String,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

#[derive(AsStd140)]
struct MaterialRaw {
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    shininess: f32,
}

impl Material {
    pub fn new(
        device: &wgpu::Device, queue: &wgpu::Queue, material: &tobj::Material,
    ) -> Result<Self> {
        // let path = format!("data/{}", material.diffuse_texture);
        let diffuse_texture = Texture::new("src/render/textures/blank.png", device, queue);
        let name = material.name.as_str().to_string();
        let material_raw = MaterialRaw {
            ambient: Vector3::from_slice(&material.ambient),
            diffuse: Vector3::from_slice(&material.diffuse),
            specular: Vector3::from_slice(&material.specular),
            shininess: material.shininess,
        };
        let buffer = MaterialExt::buffer(&name, &material_raw, device);
        let (bind_group_layout, bind_group) = MaterialExt::layout(&name, &buffer, &device);

        Ok(Self {
            diffuse_texture,
            name,
            buffer,
            bind_group_layout,
            bind_group,
        })
    }
}

struct MaterialExt;
impl MaterialExt {
    pub fn buffer(
        name: &String, material_raw: &MaterialRaw, device: &wgpu::Device,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} material buffer", name).as_str()),
            contents: material_raw.as_std140().as_bytes(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        })
    }

    pub fn layout(
        name: &String, buffer: &wgpu::Buffer, device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some(format!("{} bind group layout", name).as_str()),
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: Some(format!("{} bind group", name).as_str()),
        });

        (layout, group)
    }
}
