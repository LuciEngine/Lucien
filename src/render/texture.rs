use anyhow::Result;
use std::rc::Rc;

#[derive(Debug)]
pub struct Texture {
    pub texture: Rc<wgpu::Texture>,
    pub size: wgpu::Extent3d,
    pub group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let diffuse_image = image::open(path).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        let dimensions = &diffuse_rgba.dimensions();

        // create texture
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let diffuse_texture = TextureExt::diffuse_texture(texture_size, device);

        match TextureExt::upload_to_gpu(&diffuse_texture, &diffuse_rgba, texture_size, queue) {
            Err(e) => {
                eprintln!("{:?}", e);
                panic!("Failed to upload texture.");
            }
            _ => {
                println!("done.");
            }
        };

        // create gpu layout
        let (view, sampler) = TextureExt::view(&diffuse_texture, device);
        let (layout, group) = TextureExt::layout(&view, &sampler, device);

        Self {
            texture: Rc::new(diffuse_texture),
            size: texture_size,
            group,
            layout,
            view,
            sampler,
        }
    }
}

struct TextureExt;

impl TextureExt {
    pub fn diffuse_texture(texture_size: wgpu::Extent3d, device: &wgpu::Device) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // SAMPLED tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("diffuse_texture"),
        })
    }

    pub fn view(
        texture: &wgpu::Texture, device: &wgpu::Device,
    ) -> (wgpu::TextureView, wgpu::Sampler) {
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        (view, sampler)
    }

    pub fn layout(
        view: &wgpu::TextureView, sampler: &wgpu::Sampler, device: &wgpu::Device,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Float,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        (layout, group)
    }

    pub fn upload_to_gpu(
        texture: &wgpu::Texture, contents: &[u8], texture_size: wgpu::Extent3d, queue: &wgpu::Queue,
    ) -> Result<()> {
        queue.write_texture(
            wgpu::TextureCopyView {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            contents,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * texture_size.width,
                rows_per_image: texture_size.height,
            },
            texture_size,
        );

        Ok(())
    }
}
