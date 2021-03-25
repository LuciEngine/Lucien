use anyhow::Result;
use std::sync::Arc;

#[derive(Debug)]
pub struct RenderTexture {
    pub texture: Arc<wgpu::Texture>,
    pub size: wgpu::Extent3d,
    pub view: wgpu::TextureView,
}

impl RenderTexture {
    pub fn new(width: u32, height: u32, device: &wgpu::Device) -> Result<Self> {
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb, // the format is required by render pipeline??
            usage: wgpu::TextureUsage::COPY_SRC | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: Some("Render Texture"),
            size,
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self {
            texture: Arc::new(texture),
            size,
            view,
        })
    }
}
