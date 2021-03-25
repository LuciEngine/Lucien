use anyhow::Result;

use crate::buffer::uniform_buffer;
use crate::gpu_data::*;
use crate::Texture;

#[derive(Debug)]
pub struct Material {
    pub diffuse_texture: Texture,
    pub name: String,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        device: &wgpu::Device, queue: &wgpu::Queue, material: &tobj::Material,
    ) -> Result<Self> {
        use std::path::Path;
        // let path = format!("data/{}", material.diffuse_texture);
        let abs = Path::new(".").join("render/src/textures/blank.png").canonicalize()?;
        let diffuse_texture = Texture::new(abs.to_str().unwrap(), device, queue);
        let name = material.name.clone();
        let raw = MaterialRaw::from_tobj(material);
        let buffer = uniform_buffer(raw.as_std140().as_bytes(), device, Some("Material Buffer"));
        let (bind_group_layout, bind_group) = MaterialExt::layout(&name.as_str(), &buffer, &device);

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
    pub fn layout(
        name: &str, buffer: &wgpu::Buffer, device: &wgpu::Device,
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
