use iced_wgpu::{wgpu, Renderer};
use iced_winit::{
    slider, Align, Color, Column, Command, Element, Length, Program, Row, Slider, Text,
};

pub struct UserInterface {
    pub background_color: Color,
    pub sliders: [slider::State; 3],
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
}

// todo do actual main ui here
impl UserInterface {
    pub fn new() -> UserInterface {
        Self {
            background_color: Color::WHITE,
            sliders: Default::default(),
        }
    }

    pub fn background_color(&self) -> wgpu::Color {
        let [r, g, b, a] = self.background_color.into_linear();
        wgpu::Color {
            r: r as f64,
            g: g as f64,
            b: b as f64,
            a: a as f64,
        }
    }
}

impl Program for UserInterface {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let [r, g, b] = &mut self.sliders;
        let background_color = self.background_color;

        let sliders = Row::new()
            .width(Length::Units(500))
            .spacing(20)
            .push(
                Slider::new(r, 0.0..=1.0, background_color.r, move |r| {
                    Message::BackgroundColorChanged(Color {
                        r,
                        ..background_color
                    })
                })
                .step(0.01),
            )
            .push(
                Slider::new(g, 0.0..=1.0, background_color.g, move |g| {
                    Message::BackgroundColorChanged(Color {
                        g,
                        ..background_color
                    })
                })
                .step(0.01),
            )
            .push(
                Slider::new(b, 0.0..=1.0, background_color.b, move |b| {
                    Message::BackgroundColorChanged(Color {
                        b,
                        ..background_color
                    })
                })
                .step(0.01),
            );

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::End)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::End)
                    .push(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(Text::new("Background color").color(Color::WHITE))
                            .push(sliders)
                            .push(
                                Text::new(format!("{:?}", background_color))
                                    .size(14)
                                    .color(Color::WHITE),
                            ),
                    ),
            )
            .into()
    }
}
