use crate::Vertex;
use anyhow::{Context, Result};
use lucien_core::resources::loader;

#[derive(Debug, Clone, Copy)]
pub enum RenderMode {
    Default,
    WireFrame,
}
#[derive(Debug, Clone, Copy)]
pub enum RenderTarget {
    SwapChain,
    RenderTexture,
}
pub struct Pipeline;

impl Into<wgpu::PrimitiveTopology> for RenderMode {
    fn into(self) -> wgpu::PrimitiveTopology {
        match self {
            RenderMode::Default => wgpu::PrimitiveTopology::TriangleList,
            RenderMode::WireFrame => wgpu::PrimitiveTopology::LineStrip,
        }
    }
}

impl Pipeline {
    pub fn textured(
        layout: &wgpu::PipelineLayout, device: &wgpu::Device, shader_name: &str,
    ) -> Result<wgpu::RenderPipeline> {
        let (vs_module, fs_module) = Pipeline::load_shaders(&device, shader_name)?;
        Ok(Pipeline::create(
            Some("raster_render_pipeline"),
            layout,
            &vs_module,
            &fs_module,
            RenderMode::Default,
            device,
        ))
    }

    pub fn wireframe(
        layout: &wgpu::PipelineLayout, device: &wgpu::Device, shader_name: &str,
    ) -> Result<wgpu::RenderPipeline> {
        let (vs_module, fs_module) = Pipeline::load_shaders(&device, shader_name)?;
        Ok(Pipeline::create(
            Some("wireframe_render_pipeline"),
            layout,
            &vs_module,
            &fs_module,
            RenderMode::WireFrame,
            device,
        ))
    }

    fn load_shaders(
        device: &wgpu::Device, shader_name: &str,
    ) -> Result<(wgpu::ShaderModule, wgpu::ShaderModule)> {
        let vs_src = loader()?
            .load_text(format!("{}.vert.glsl", shader_name).as_str())
            .context("Failed to load vert shader")?;
        let fs_src = loader()?
            .load_text(format!("{}.frag.glsl", shader_name).as_str())
            .context("Failed to load frag shader")?;
        let mut compiler = shaderc::Compiler::new().context("Failed to compile shader")?;
        let vs_spirv = compiler
            .compile_into_spirv(
                vs_src.as_str(),
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
                fs_src.as_str(),
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
        Ok((vs_module, fs_module))
    }

    // todo accept config
    fn create(
        label: Option<&str>, layout: &wgpu::PipelineLayout, vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule, mode: RenderMode, device: &wgpu::Device,
    ) -> wgpu::RenderPipeline {
        let desc = &wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
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
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: mode.into(),
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
        };
        device.create_render_pipeline(desc)
    }
}
