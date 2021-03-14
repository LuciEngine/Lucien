use wgpu;
use wgpu::util::DeviceExt;

use super::raw_data::*;
use crate::render::Scene;

#[derive(Debug)]
pub struct Uniforms {
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Uniforms {
    // This sends data once, if we want to update, need to use copy data to buffer
    // Copy data is done in update_buffer
    pub fn new(scene: &Scene, device: &wgpu::Device) -> Self {
        let buffer = UniformsExt::buffer(UniformsRaw::from(&scene.camera), device);
        let (bind_group_layout, bind_group) = UniformsExt::layout(&buffer, device);

        Uniforms {
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    // create a buffer contains latest data, that we need to use a buffer to send data
    // copy the buffer to previously created uniforms buffer
    pub fn update_buffer(
        &self, scene: &Scene, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device,
    ) {
        let buffer = UniformsExt::buffer(UniformsRaw::from(&scene.camera), device);
        let buffer_size = std::mem::size_of::<UniformsRaw>() as wgpu::BufferAddress;
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.buffer, 0, buffer_size);
    }
}

struct UniformsExt;
impl UniformsExt {
    pub fn buffer(raw: UniformsRaw, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&[raw]),
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
            label: Some("uniform_bind_group_layout"),
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });
        (layout, group)
    }
}
