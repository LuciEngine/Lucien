use anyhow::{Context, Result};
use wgpu;
use winit::window::*;

use crate::render::*;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
    light: Light,
    scene: Scene,
    depth_texture: DepthTexture,
}

impl State {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();

        // initializing GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .context("Failed to create adapter")?;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await?;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // 上面统统都没有

        // load models and materials, set up scene + light + uniforms
        let scene = Scene::new().load("src/examples/data/cube.obj", &device, &queue);
        let uniforms = Uniforms::default(&device);
        let light = Light::default(&device);

        let mut bind_group_layouts = vec![&uniforms.bind_group_layout];
        bind_group_layouts.push(
            &scene.materials[scene.models[0].mesh.material]
                .diffuse_texture
                .layout,
        );
        bind_group_layouts.push(&scene.materials[scene.models[0].mesh.material].bind_group_layout);
        bind_group_layouts.push(&light.bind_group_layout);

        // load shaders
        let (vs_module, fs_module) = StateExt::load_shaders(&device);
        // create depth texture
        let depth_texture = DepthTexture::new(&device, &sc_desc, Some("depth_texture"));

        // render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &bind_group_layouts[..],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("raster_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        // creating buffers;
        Ok(Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            uniforms,
            light,
            scene,
            depth_texture,
        })
    }

    pub fn resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {
        // self.size = new_size;
        // self.sc_desc.width = new_size.width;
        // self.sc_desc.height = new_size.height;
        // self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        let mut encoder = StateExt::create_encoder(Some("Update Encoder"), &self.device);
        // actual update
        // self.uniforms.camera.eye.z -= 0.01;
        self.uniforms.camera.update_view_matrix();
        self.uniforms.update_buffer(&mut encoder, &self.device);
        // commit changes
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = StateExt::create_encoder(Some("Render Encoder"), &self.device);
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
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
        let material = &self.scene.materials[self.scene.models[0].mesh.material];
        let mesh = &self.scene.models[0].mesh;

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniforms.bind_group, &[]);
        render_pass.set_bind_group(1, &material.diffuse_texture.group, &[]);
        render_pass.set_bind_group(2, &material.bind_group, &[]);
        render_pass.set_bind_group(3, &self.light.bind_group, &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

struct StateExt;

impl StateExt {
    pub fn create_encoder(label: Option<&str>, device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
    }

    pub fn load_shaders(device: &wgpu::Device) -> (wgpu::ShaderModule, wgpu::ShaderModule) {
        let vs_src = include_str!("shaders/shader.vert.glsl");
        let fs_src = include_str!("shaders/shader.frag.glsl");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler
            .compile_into_spirv(
                vs_src,
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
                fs_src,
                shaderc::ShaderKind::Fragment,
                "shader.frag",
                "main",
                None,
            )
            .unwrap();
        let vs_data = wgpu::util::make_spirv(vs_spirv.as_binary_u8());
        let fs_data = wgpu::util::make_spirv(fs_spirv.as_binary_u8());
        let vs_module = device.create_shader_module(vs_data);
        let fs_module = device.create_shader_module(fs_data);
        (vs_module, fs_module)
    }
}
