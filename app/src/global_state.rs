use crate::message::Message;
use crate::*;

use anyhow::Result;
use futures::executor::block_on;
use iced_wgpu::{wgpu, Viewport};
use iced_winit::{futures, winit, Size};
use winit::event_loop::EventLoop;

#[derive(Debug)]
pub(crate) struct GlobalState {
    pub window: winit::window::Window,
    pub viewport: Viewport,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub sc: wgpu::SwapChain,
    pub resized: bool,
}

impl GlobalState {
    pub fn new(event_loop: &EventLoop<Message>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        let size = window.inner_size();
        let viewport =
            Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());
        let (device, queue, surface, sc) = block_on(init_with_window(&window)).unwrap();

        Self {
            window,
            viewport,
            device,
            queue,
            surface,
            sc,
            resized: false,
        }
    }

    pub fn get_size(&self) -> [u32; 2] {
        let size = self.window.inner_size();
        [size.width, size.height]
    }

    // recalculate viewport
    pub fn viewport(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        let viewport = Viewport::with_physical_size(
            Size::new(size.width, size.height),
            self.window.scale_factor(),
        );
        self.viewport = viewport;
    }

    pub fn resize(&mut self) -> Result<()> {
        self.sc = create_swap_chain(&self.window, &self.device, &self.surface)?;
        Ok(())
    }
}
