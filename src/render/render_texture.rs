use std::rc::Rc;

#[derive(Debug)]
pub struct RenderTexture {
    pub texture: Rc<wgpu::Texture>,
    pub size: wgpu::Extent3d,
    pub view: wgpu::TextureView,
}

impl RenderTexture {
    pub fn new(width: u32, height: u32, device: &wgpu::Device) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let texture_desc = wgpu::TextureDescriptor {
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::COPY_SRC | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
            size,
        };
        let texture = device.create_texture(&texture_desc);
        let view = texture.create_view(&Default::default());

        Self {
            texture: Rc::new(texture),
            size,
            view,
        }
    }
}

struct RenderTextureExt;
impl RenderTextureExt {}
