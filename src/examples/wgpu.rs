use crate::core::message::Message;
use crate::render::*;
use anyhow::{Context, Result};
use futures::executor::block_on;
use iced::scrollable;
use iced::{Align, Column, Container, Length, Scrollable, Text};

pub fn container(scroll: &mut scrollable::State) -> iced::Element<'_, Message> {
    let im = cube_image();

    let content = Column::new()
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Align::Start)
        .push(im)
        .push(Text::new("Render image using 3D renderer."));

    let scrollable =
        Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

    let container = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y();

    container.into()
}

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

pub fn cube_image() -> iced::Image {
    let size = [64, 64];
    let (device, queue) = block_on(init_gpu_headless()).unwrap();

    let render_settings = RenderSettings::new();
    let mut renderer = Renderer::new(device, queue, size).unwrap();

    renderer.render(&render_settings).unwrap();
    renderer.update();
    renderer.read_to_buffer().unwrap();

    let raw = block_on(renderer.as_rgba()).unwrap();
    let img = iced::image::Handle::from_memory(raw.to_vec());

    // raw.save("cube.png").unwrap();

    iced::Image::new(img)
        .width(Length::Units(size[0] as u16))
        .height(Length::Units(size[1] as u16))
}
