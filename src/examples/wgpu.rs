use anyhow::{Context, Result};
use futures::executor::block_on;

use crate::render::*;

async fn init_gpu_headless() -> Result<(wgpu::Device, wgpu::Queue)> {
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

    Ok((device, queue))
}

pub fn main() {
    let size = [512, 512];
    let (device, queue) = block_on(init_gpu_headless()).unwrap();

    let render_settings = RenderSettings::new();
    let mut renderer = Renderer::new(device, queue, size).unwrap();

    renderer.render(&render_settings).unwrap();
    renderer.update();
    renderer.read_to_buffer().unwrap();

    block_on(renderer.save_png("cube")).unwrap();
}
