use anyhow::{Context, Result};

use iced_wgpu::wgpu;
use iced_winit::winit;

mod backend;
mod frontend;
mod global_state;

use backend::*;
use frontend::*;
use global_state::*;

pub mod application;
pub mod message;
pub mod widgets;

#[allow(dead_code)]
async fn init_headless() -> Result<(wgpu::Device, wgpu::Queue)> {
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

async fn init_with_window(
    window: &winit::window::Window,
) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface, wgpu::SwapChain)> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(window) };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .context("Request adapter")?;

    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .context("Failed to request device")?;

    let swap_chain =
        create_swap_chain(&window, &device, &surface).context("Failed to create swap_chain")?;

    Ok((device, queue, surface, swap_chain))
}

// Resize swap chain texture size
fn create_swap_chain(
    window: &winit::window::Window, device: &wgpu::Device, surface: &wgpu::Surface,
) -> Result<wgpu::SwapChain> {
    let swap_chain = {
        let size = window.inner_size();

        device.create_swap_chain(
            surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        )
    };
    Ok(swap_chain)
}
