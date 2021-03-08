use iced::scrollable;
use iced::{Align, Column, Container, Length, Scrollable, Text};

use crate::core::message::Message;
use crate::examples::rainbow::Rainbow;

#[allow(dead_code)]
pub fn container(scroll: &mut scrollable::State) -> iced::Element<'_, Message> {
    let content = Column::new()
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Align::Start)
        .push(Rainbow::new())
        .push(Text::new("The demo uses iced API to draw 2D mesh."));

    let scrollable =
        Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

    let container = Container::new(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y();

    container.into()
}
