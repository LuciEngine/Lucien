use glam::{vec3, Vec3};
use wgpu;
use wgpu::util::DeviceExt;

use super::raw_data::*;

// Point Light
#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    // set a bound?
    pub color: Vec3,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, device: &wgpu::Device) -> Self {
        let buffer = LightExt::buffer(LightRaw::from_vec3(&position, &color), device);
        let (bind_group_layout, bind_group) = LightExt::layout(&buffer, device);

        Light {
            position,
            color,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn default(device: &wgpu::Device) -> Self {
        let position = vec3(0.7, 0.0, 2.0);
        let color = vec3(0.1, 0.1, 0.1);

        Light::new(position, color, device)
    }

    // create a buffer contains latest data, that we need to use a buffer to send data
    // copy the buffer to previously created light buffer
    pub fn update_buffer(&self, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device) {
        let buffer = LightExt::buffer(LightRaw::from_vec3(&self.position, &self.color), device);
        let buffer_size = std::mem::size_of::<LightRaw>() as wgpu::BufferAddress;
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.buffer, 0, buffer_size);
    }
}

struct LightExt;
impl LightExt {
    pub fn buffer(raw: LightRaw, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: raw.as_std140().as_bytes(),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        })
    }

    pub fn layout(
        buffer: &wgpu::Buffer, device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("light_bind_group_layout"),
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: Some("light_bind_group"),
        });

        (layout, group)
    }
}
