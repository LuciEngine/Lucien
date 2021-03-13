use iced::scrollable;
use iced::widget::image;
use iced::{Align, Column, Container, Image, Length, Scrollable, Text};

use crate::application::EngineApp;
use crate::core::message::Message;
use crate::examples::bunny_raster;

mod style {
    use iced::{container, Background, Color};

    pub enum Box {
        Primary,
    }

    impl container::StyleSheet for Box {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(match self {
                    Box::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_color: Color::default(),
                border_width: 0.0,
                border_radius: 0.0,
                text_color: Some(Color::BLACK),
            }
        }
    }
}

pub fn bunny<'a>(_: u16, app: &EngineApp) -> Container<'a, Message> {
    let raw = bunny_raster::render(app).unwrap();
    Container::new(
        Image::new(image::Handle::from_memory(raw))
            .width(Length::Units(500))
            .height(Length::Units(500)),
    )
    .style(style::Box::Primary)
    .width(Length::Fill)
    .center_x()
}

pub fn container<'a>(
    scroll: &'a mut scrollable::State,
    bunny: Container<'a, Message>,
) -> iced::Element<'a, Message> {
    let content = Column::new()
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Align::Start)
        .push(bunny)
        .push(Text::new("The demo uses builtin raster to render a bunny."));

    let scrollable =
        Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

    let container = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y();

    container.into()
}
