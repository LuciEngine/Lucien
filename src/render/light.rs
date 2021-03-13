use wgpu;
use wgpu::util::DeviceExt;
use bytemuck::{ Pod, Zeroable };

#[derive(Debug)]
pub struct Light {
    pub position: [f32; 3],
    pub color: [f32; 3],
    _padding: u32,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct LightRaw {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub padding: f32,
}

unsafe impl Zeroable for LightRaw {}
unsafe impl Pod for LightRaw {}

// Point Light
impl Light {
    pub fn new(position: [f32; 3], color: [f32; 3], device: &wgpu::Device) -> Self {
        let raw = LightRaw { position, color, padding: 0.0 };
        let buffer = LightExt::buffer(raw, device);
        let (bind_group_layout, bind_group) = LightExt::layout(&buffer, device);

        Light {
            position,
            color,
            _padding: 0,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn default(device: &wgpu::Device) -> Self {
        let position = [0.0, 1.0, 2.0];
        let color = [1.0, 1.0, 1.0];

        Light::new(position, color, device)
    }
}

struct LightExt;
impl LightExt {
    pub fn buffer(raw: LightRaw, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[raw]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
    }

    pub fn layout(buffer: &wgpu::Buffer, device: &wgpu::Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
                    },
                ],
            label: None,
        });

        (layout, group)
    }
}
