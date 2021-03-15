use glam::{vec3, Vec3};

use super::buffer::uniform_buffer;
use super::gpu_data::*;

// Point Light
#[derive(Debug)]
pub struct PointLight {
    pub position: Vec3,
    // set a bound?
    pub color: Vec3,
    pub intensity: f32,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

// Ambient light is used in uniform buffer, so it's
// not created with a layout
#[derive(Debug)]
pub struct AmbientLight {
    pub color: Vec3,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Vec3, device: &wgpu::Device) -> Self {
        let intensity = 1.0;
        let raw = PointLightRaw::from_vec3(&position, &color, intensity);
        let buffer = uniform_buffer(
            raw.as_std140().as_bytes(),
            device,
            Some("Point Light Buffer"),
        );
        let (bind_group_layout, bind_group) = PointLightExt::layout(&buffer, device);

        PointLight {
            position,
            intensity,
            color,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn default(device: &wgpu::Device) -> Self {
        let position = vec3(0.7, 0.0, 2.0);
        let color = vec3(0.1, 0.1, 0.1);
        PointLight::new(position, color, device)
    }

    // create a buffer contains latest data, that we need to use a buffer to send data
    // copy the buffer to previously created light buffer
    pub fn update_buffer(&self, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device) {
        let raw = PointLightRaw::from_vec3(&self.position, &self.color, self.intensity);
        let buffer = uniform_buffer(
            raw.as_std140().as_bytes(),
            device,
            Some("Point Light Buffer"),
        );
        let buffer_size = std::mem::size_of::<PointLightRaw>() as wgpu::BufferAddress;
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.buffer, 0, buffer_size);
    }
}

impl AmbientLight {
    pub fn new(color: Vec3) -> Self {
        let intensity = 1.0;

        Self { color, intensity }
    }

    pub fn default() -> Self {
        AmbientLight::new(vec3(1.0, 1.0, 1.0))
    }
}

struct PointLightExt;
impl PointLightExt {
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
