use iced_graphics::{
    triangle::{Mesh2D, Vertex2D},
    Backend, Defaults, Primitive, Renderer,
};
use iced_native::{
    layout, mouse, Element, Hasher, Layout, Length, Point, Rectangle, Size, Vector, Widget,
};

pub struct Rainbow;

impl Rainbow {
    pub fn new() -> Self {
        Self
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for Rainbow
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        let size = limits.width(Length::Fill).resolve(Size::ZERO);

        layout::Node::new(Size::new(size.width, size.width))
    }

    fn hash_layout(&self, _state: &mut Hasher) {}

    fn draw(
        &self, _renderer: &mut Renderer<B>, _defaults: &Defaults, layout: Layout<'_>,
        cursor_position: Point, _viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        let b = layout.bounds();

        // R O Y G B I V
        let color_r = [1.0, 0.0, 0.0, 1.0];
        let color_o = [1.0, 0.5, 0.0, 1.0];
        let color_y = [1.0, 1.0, 0.0, 1.0];
        let color_g = [0.0, 1.0, 0.0, 1.0];
        let color_gb = [0.0, 1.0, 0.5, 1.0];
        let color_b = [0.0, 0.2, 1.0, 1.0];
        let color_i = [0.5, 0.0, 1.0, 1.0];
        let color_v = [0.75, 0.0, 0.5, 1.0];

        let posn_center = {
            if b.contains(cursor_position) {
                [cursor_position.x - b.x, cursor_position.y - b.y]
            } else {
                [b.width / 2.0, b.height / 2.0]
            }
        };

        let posn_tl = [0.0, 0.0];
        let posn_t = [b.width / 2.0, 0.0];
        let posn_tr = [b.width, 0.0];
        let posn_r = [b.width, b.height / 2.0];
        let posn_br = [b.width, b.height];
        let posn_b = [(b.width / 2.0), b.height];
        let posn_bl = [0.0, b.height];
        let posn_l = [0.0, b.height / 2.0];

        (
            Primitive::Translate {
                translation: Vector::new(b.x, b.y),
                content: Box::new(Primitive::Mesh2D {
                    size: b.size(),
                    buffers: Mesh2D {
                        vertices: vec![
                            Vertex2D {
                                position: posn_center,
                                color: [1.0, 1.0, 1.0, 1.0],
                            },
                            Vertex2D {
                                position: posn_tl,
                                color: color_r,
                            },
                            Vertex2D {
                                position: posn_t,
                                color: color_o,
                            },
                            Vertex2D {
                                position: posn_tr,
                                color: color_y,
                            },
                            Vertex2D {
                                position: posn_r,
                                color: color_g,
                            },
                            Vertex2D {
                                position: posn_br,
                                color: color_gb,
                            },
                            Vertex2D {
                                position: posn_b,
                                color: color_b,
                            },
                            Vertex2D {
                                position: posn_bl,
                                color: color_i,
                            },
                            Vertex2D {
                                position: posn_l,
                                color: color_v,
                            },
                        ],
                        indices: vec![
                            0, 1, 2, // TL
                            0, 2, 3, // T
                            0, 3, 4, // TR
                            0, 4, 5, // R
                            0, 5, 6, // BR
                            0, 6, 7, // B
                            0, 7, 8, // BL
                            0, 8, 1, // L
                        ],
                    },
                }),
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for Rainbow
where
    B: Backend,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}
