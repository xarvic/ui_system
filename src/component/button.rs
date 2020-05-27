use crate::component::component::Component;
use crate::renderer::{CommandBuffer, Builder};
use core::position::Vector;
use core::color::Color;

pub struct Button {
    inner: Option<Box<dyn Component>>,
    on_click: Option<Box<FnMut()>>
}

impl Button{
    fn new() -> Button{
        Button{
            inner: None,
            on_click: None,
        }
    }
    #[inline]
    fn onclick(mut self, handler: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Component for Button {
    fn size(&self) -> Vector{
        Vector::new(100.0, 40.0)
    }

    fn pref_size(&self) -> Vector{
        self.size()
    }

    fn build(&mut self, mut buffer: Builder) {
        buffer.draw_round_rect(Vector::null(), self.size(), Color::new(0.5, 0.5, 1.0, 1.0), [10.0, 10.0, 10.0, 10.0]);
    }
}

pub fn button() -> Button{
    Button::new()
}