use crate::render::buffer::render_buffer;
use crate::render::*;

use anyhow::{Context, Result};
use futures::executor::block_on;

async fn init_gpu_headless(
    size: &[u32; 2],
) -> Result<(wgpu::Device, wgpu::Queue, RenderTexture, wgpu::Buffer)> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        })
        .await
        .context("Failed to request adapter")?;
    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .context("Failed to request device")?;
    let rt =
        RenderTexture::new(size[0], size[1], &device).context("Failed to create render texture")?;
    let rb = render_buffer(&rt, &device);

    Ok((device, queue, rt, rb))
}

// Same thing for iced ui, all we need is state.render()
pub fn main() {
    let size = [256, 256];
    let (device, queue, rt, rb) = block_on(init_gpu_headless(&size)).unwrap();

    let render_settings = RenderSettings::new(rt);
    let mut renderer = Renderer::new(device, queue, size, Some(rb)).unwrap();

    renderer.render(&render_settings).unwrap();
    renderer.update();
    renderer.read_to_buffer(&render_settings).unwrap();
}
