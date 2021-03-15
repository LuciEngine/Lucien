use anyhow::{Context, Result};
use wgpu;

use crate::render::*;

// Render to render_texture, and you can read it to render_buffer
#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub render_mode: RenderMode,
    pub clear_color: Option<wgpu::Color>,
}
// Renderer accepts a RenderSettings, writes data to a RenderState
// rt is a render texture (render target)
// rb is a render buffer, can read data from the texture
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
    state: RenderState,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, size: [u32; 2]) -> Result<Self> {
        let state =
            RenderState::new(size, &device, &queue).context("Failed to create render state")?;
        // first model & material
        let mesh = &state.scene.models[0].mesh;
        let material = &state.scene.materials[mesh.material];

        let mut bind_group_layouts = vec![&state.uniforms.bind_group_layout];
        bind_group_layouts.push(&material.diffuse_texture.layout);
        bind_group_layouts.push(&material.bind_group_layout);
        bind_group_layouts.push(&state.scene.light.bind_group_layout);

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
            state,
        })
    }

    pub fn update(&mut self) {
        let mut encoder = Renderer::create_encoder(Some("Update Encoder"), &self.device);
        // update scene
        self.state.scene.camera.eye.z -= 0.01;
        self.state.scene.camera.update_view_matrix();
        self.state.scene.light.position = self.state.scene.camera.eye;

        self.state
            .uniforms
            .update_buffer(&self.state.scene, &mut encoder, &self.device);
        self.state
            .scene
            .light
            .update_buffer(&mut encoder, &self.device);

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render(&mut self, settings: &RenderSettings) -> Result<()> {
        let clear = settings.get_clear_color();
        let mut encoder = Renderer::create_encoder(Some("Render Encoder"), &self.device);
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                // write colors to render target
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.state.rt.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: true,
                    },
                }],
                // write z-values to depth texture
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.state.depth.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            // render first material for first mesh
            let mesh = &self.state.scene.models[0].mesh;
            let material = &self.state.scene.materials[mesh.material];

            render_pass.set_pipeline(&self.textured_pipeline);
            render_pass.set_bind_group(0, &self.state.uniforms.bind_group, &[]);
            render_pass.set_bind_group(1, &material.diffuse_texture.group, &[]);
            render_pass.set_bind_group(2, &material.bind_group, &[]);
            render_pass.set_bind_group(3, &self.state.scene.light.bind_group, &[]);
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

    pub fn read_to_buffer(&self) -> Result<()> {
        assert_eq!(
            self.state.rb.is_none(),
            false,
            "Render Buffer not set, can't read render result to buffer."
        );
        let u32_size = std::mem::size_of::<u32>() as u32;
        let mut encoder = Renderer::create_encoder(Some("Render Buffer Encoder"), &self.device);
        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: &self.state.rt.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &self.state.rb.as_ref().unwrap(),
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: u32_size * self.state.rt.size.width,
                    rows_per_image: self.state.rt.size.height,
                },
            },
            self.state.rt.size,
        );
        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}

impl RenderSettings {
    pub fn new() -> Self {
        Self {
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
