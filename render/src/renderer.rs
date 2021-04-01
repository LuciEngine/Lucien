use crate::{DepthTexture, Pipeline, RenderMode, RenderTarget, RenderTexture, Scene, Uniforms};
use anyhow::{Context, Result};
use time::Instant;

pub type RgbaBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub size: [u32; 2],
    pub render_mode: RenderMode,
    pub render_target: RenderTarget,
    pub clear_color: Option<wgpu::Color>,
}
// Renderer accepts a RenderSettings, writes data to a RenderState
// rt is a render texture (render target)
// rb is a render buffer, can read data from the texture
#[derive(Debug)]
pub struct RenderState {
    pub rt: RenderTexture,
    pub rb: Option<wgpu::Buffer>,
    pub size: [u32; 2],
    pub start_at: Instant,
    depth: DepthTexture,
    uniforms: Uniforms,
    scene: Scene,
}

// Render to render texture, and you can read it to render buffer.
// We determine which pipeline is used using render settings.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Renderer {
    pub size: [u32; 2],
    pub textured_pipeline: wgpu::RenderPipeline,
    pub wireframe_pipeline: wgpu::RenderPipeline,
    pub state: RenderState,
}

// Send for indicating wwnership may be transferred to another thread,
// We do render in a separate thread so it doesn't block the frontend.
// See:
//  https://itfanr.gitbooks.io/rust-book-2rd-en/content/ch16-04-extensible-concurrency-sync-and-send.html
unsafe impl Send for Renderer {}

impl Renderer {
    // use first model & material to create pipeline memory layout
    pub fn new(
        device: &wgpu::Device, queue: &wgpu::Queue, settings: &RenderSettings,
    ) -> Result<Self> {
        let size = settings.size;
        let state =
            RenderState::new(size, &device, &queue).context("Failed to create render state")?;
        let mesh = &state.scene.models[0].mesh;
        let material = &state.scene.materials[mesh.material];

        let mut bind_group_layouts = vec![&state.uniforms.bind_group_layout];
        bind_group_layouts.push(&material.diffuse_texture.layout);
        bind_group_layouts.push(&material.bind_group_layout);
        bind_group_layouts.push(&state.scene.light.bind_group_layout);

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
            // device,
            // queue,
            textured_pipeline,
            wireframe_pipeline,
            state,
        })
    }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = self.create_encoder(Some("Update Encoder"), device);

        // update the game state here
        // todo replace with actual game logic
        let time = self.state.start_at.elapsed().as_seconds_f32();

        self.state.scene.camera.eye.x = time.sin() * 5.0;
        self.state.scene.camera.eye.z = time.cos() * 5.0;
        self.state.scene.camera.update_view_matrix();
        self.state.scene.light.position = self.state.scene.camera.eye;

        self.state
            .uniforms
            .update_buffer(&self.state.scene, &mut encoder, device);
        self.state.scene.light.update_buffer(&mut encoder, device);

        queue.submit(std::iter::once(encoder.finish()));
    }

    // Takes render settings, uses data in render state, and writes to a
    // render texture that we set up earlier in the render state
    pub fn render(
        &self, settings: &RenderSettings, device: &wgpu::Device, queue: &wgpu::Queue,
    ) -> Result<()> {
        let mut encoder = self.create_encoder(Some("Render Encoder"), device);
        {
            let mut render_pass = self.create_render_pass(settings, &mut encoder);
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
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    // Render to external target, instead of self.state.rt
    pub fn render_external(
        &self, target: &wgpu::SwapChainTexture, settings: &RenderSettings, device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        let mut encoder = self.create_encoder(Some("Render Encoder"), device);
        {
            let mut render_pass = self.create_render_pass_external(target, settings, &mut encoder);
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
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    // Save render result from render texture to render buffer,
    // it is made explicit because we don't want to write buffer on every render.
    pub fn read_to_buffer(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<()> {
        assert_eq!(
            self.state.rb.is_none(),
            false,
            "Render Buffer not set, can't read render result to buffer."
        );
        let u32_size = std::mem::size_of::<u32>() as u32;
        let mut encoder = self.create_encoder(Some("Render Buffer Encoder"), device);
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
        queue.submit(Some(encoder.finish()));

        Ok(())
    }

    // An encoder is used to submit commands to gpu.
    pub fn create_encoder(
        &self, label: Option<&str>, device: &wgpu::Device,
    ) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
    }

    // Create a render pass, you are subjective to call encoder.finish after this
    fn create_render_pass<'a>(
        &'a self, settings: &RenderSettings, encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        let clear = settings.get_clear_color();
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        render_pass
    }

    // Create render pass for external render target
    fn create_render_pass_external<'a>(
        &'a self, target: &'a wgpu::SwapChainTexture, settings: &RenderSettings,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        let clear = settings.get_clear_color();
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            // write colors to render target
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &target.view,
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
        render_pass
    }

    // Convert current render result from render buffer to rgba
    pub async fn as_rgba(&self, device: &wgpu::Device) -> Result<RgbaBuffer> {
        self.state.as_rgba(device).await
    }
}

impl RenderSettings {
    pub fn new(size: [u32; 2]) -> Self {
        Self {
            size,
            render_target: RenderTarget::RenderTexture,
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

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings::new([1024, 1024])
    }
}

impl RenderState {
    pub fn new(size: [u32; 2], device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self> {
        use super::buffer::*;

        let scene = Scene::new(device).load("src/examples/data/cube.obj", device, queue)?;
        let uniforms = Uniforms::new(&scene, device);
        let depth = DepthTexture::new(device, size[0], size[1], Some("depth_texture"));
        let rt = RenderTexture::new(size[0], size[1], device)
            .context("Failed to create render texture")?;
        let rb = Some(render_buffer(&rt, device));
        let start_at = Instant::now();

        Ok(Self {
            rt,
            rb,
            size,
            uniforms,
            depth,
            scene,
            start_at,
        })
    }

    pub fn resize(&mut self, size: [u32; 2], device: &wgpu::Device) -> Result<()> {
        self.depth = DepthTexture::new(device, size[0], size[1], Some("depth_texture"));
        self.rt = RenderTexture::new(size[0], size[1], device)
            .context("Failed to create render texture")?;

        Ok(())
    }

    // This function is slow and when executor has multiple tasks running it,
    // since the operation is not finished, some tasks will panic and fail.
    async fn as_rgba(&self, device: &wgpu::Device) -> Result<RgbaBuffer> {
        use image::buffer::ConvertBuffer;
        use image::{Bgra, ImageBuffer, Rgba};

        let buffer: ImageBuffer<Rgba<u8>, _>;
        {
            // we have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let buffer_slice = self.rb.as_ref().unwrap().slice(..);
            let mapping = buffer_slice.map_async(wgpu::MapMode::Read);

            // We don't block the buffer, otherwise it's very slow
            device.poll(wgpu::Maintain::Wait);
            mapping.await.unwrap();

            let data = buffer_slice.get_mapped_range();
            // convert render texture from bgra to rgba
            // render texture is bgra by default, required by wgpu low level
            let width = self.size[0];
            let height = self.size[1];
            let raw = ImageBuffer::<Bgra<u8>, _>::from_raw(width, height, data).unwrap();

            buffer = raw.convert();
        }
        self.rb.as_ref().unwrap().unmap();

        Ok(buffer)
    }
}
