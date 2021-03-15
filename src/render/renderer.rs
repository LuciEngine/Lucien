use anyhow::{Context, Result};
use wgpu;

use crate::render::*;

// Render to render_texture, and you can read it to render_buffer
pub struct RenderSettings {
    pub render_texture: RenderTexture,
    pub render_mode: RenderMode,
    pub clear_color: Option<wgpu::Color>,
}
// Renderer takes a RenderSettings, writes data to a RenderState
// rt is a render texture (render target)
// rb is a render buffer, can read data from the texture
#[allow(dead_code)]
pub struct RenderState {
    pub rt: RenderTexture,
    pub rb: Option<wgpu::Buffer>,
    pub size: [u32; 2],
    depth: DepthTexture,
    uniforms: Uniforms,
    scene: Scene,
}

#[allow(dead_code)]
pub struct Renderer {
    pub size: [u32; 2],
    device: wgpu::Device,
    queue: wgpu::Queue,
    textured_pipeline: wgpu::RenderPipeline,
    wireframe_pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
    scene: Scene,
    depth_texture: DepthTexture,
    render_buffer: Option<wgpu::Buffer>,
    // state: RenderState,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device, queue: wgpu::Queue, size: [u32; 2], rb: Option<wgpu::Buffer>,
    ) -> Result<Self> {
        // set up scene
        let scene = Scene::new(&device).load("src/examples/data/cube.obj", &device, &queue);
        // create buffer
        let uniforms = Uniforms::new(&scene, &device);
        // create depth texture
        let depth_texture = DepthTexture::new(&device, size[0], size[1], Some("depth_texture"));

        // first model & material
        let mesh = &scene.models[0].mesh;
        let material = &scene.materials[mesh.material];

        let mut bind_group_layouts = vec![&uniforms.bind_group_layout];
        bind_group_layouts.push(&material.diffuse_texture.layout);
        bind_group_layouts.push(&material.bind_group_layout);
        bind_group_layouts.push(&scene.light.bind_group_layout);

        // render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &bind_group_layouts[..],
                push_constant_ranges: &[],
            });
        let textured_pipeline = Pipeline::textured(&render_pipeline_layout, &device);
        let wireframe_pipeline = Pipeline::wireframe(&render_pipeline_layout, &device);

        Ok(Self {
            size,
            device,
            queue,
            textured_pipeline,
            wireframe_pipeline,
            uniforms,
            scene,
            depth_texture,
            render_buffer: rb,
        })
    }

    pub fn update(&mut self) {
        let mut encoder = Renderer::create_encoder(Some("Update Encoder"), &self.device);
        // update scene
        self.scene.camera.eye.z -= 0.01;
        self.scene.camera.update_view_matrix();
        self.scene.light.position = self.scene.camera.eye;

        self.uniforms
            .update_buffer(&self.scene, &mut encoder, &self.device);
        self.scene.light.update_buffer(&mut encoder, &self.device);

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render(&mut self, settings: &RenderSettings) -> Result<(), wgpu::SwapChainError> {
        let clear = settings.get_clear_color();
        let mut encoder = Renderer::create_encoder(Some("Render Encoder"), &self.device);
        {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            // write colors to render target
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &settings.render_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: true,
                },
            }],
            // write z-values to depth texture
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
            // render first material for first mesh
            let mesh = &self.scene.models[0].mesh;
            let material = &self.scene.materials[mesh.material];

            render_pass.set_pipeline(&self.textured_pipeline);
            render_pass.set_bind_group(0, &self.uniforms.bind_group, &[]);
            render_pass.set_bind_group(1, &material.diffuse_texture.group, &[]);
            render_pass.set_bind_group(2, &material.bind_group, &[]);
            render_pass.set_bind_group(3, &self.scene.light.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..));
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    fn create_encoder(label: Option<&str>, device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
    }

    pub fn read_to_buffer(&self, settings: &RenderSettings) -> Result<()> {
        assert_eq!(
            self.render_buffer.is_none(),
            false,
            "Render Buffer not set, can't read render result to buffer."
        );
        let u32_size = std::mem::size_of::<u32>() as u32;
        let mut encoder = Renderer::create_encoder(Some("Render Buffer Encoder"), &self.device);
        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: &settings.render_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &self.render_buffer.as_ref().unwrap(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: u32_size * settings.render_texture.size.width,
                    rows_per_image: settings.render_texture.size.height,
                },
            },
            settings.render_texture.size,
        );
        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

impl RenderSettings {
    pub fn new(render_texture: RenderTexture) -> Self {
        Self {
            render_texture,
            render_mode: RenderMode::Default,
            clear_color: None,
        }
    }

    pub fn get_clear_color(&self) -> wgpu::Color {
        self.clear_color.unwrap_or(wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        })
    }
}

#[allow(dead_code)]
impl RenderState {
    pub fn new(size: [u32; 2], device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self> {
        use super::buffer::*;

        let scene = Scene::new(device).load("src/examples/data/cube.obj", device, queue);
        let uniforms = Uniforms::new(&scene, device);
        let depth = DepthTexture::new(device, size[0], size[1], Some("depth_texture"));
        let rt = RenderTexture::new(size[0], size[1], device)
            .context("Failed to create render texture")?;
        let rb = Some(render_buffer(&rt, device));

        Ok(Self {
            rt,
            rb,
            size,
            uniforms,
            depth,
            scene,
        })
    }
}
