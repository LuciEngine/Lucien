use crate::application::State;
use crate::core::message::Message;
use iced::{Align, Column, Container, Length, Text};

pub fn main_window(state: &State) -> iced::Element<'static, Message> {
    // load image from disk,
    // we are waiting for iced to load image from memory
    // the feature is in new version, with wgpu 0.7.0
    let im = iced::Image::new("window.png");

    let content = Column::new()
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Align::Start)
        .push(im)
        .push(Text::new("Render image using 3D renderer."))
        .push(Text::new(format!("ticks: {:?}", state.ticks)));

    let container = Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

    container.into()
}
