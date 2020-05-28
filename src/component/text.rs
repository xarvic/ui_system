use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;
use crate::component::event::Event;
use core::position::Vector;
use core::color::{Color, BLACK};

pub struct Text{
    text: String,
    size: f32,
    display_length: Option<f32>,
    color: Color,
}

impl Text {
    fn new(text: String) -> Text {
        Text{text, size: 20.0, display_length: None, color: BLACK}
    }
    fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }


    fn width(&self) -> f32 {
        (self.size * 36.0 / 70.0).floor()
    }
}

impl Component for Text {
    fn size(&self) -> Vector {
        self.pref_size()
    }

    fn pref_size(&self) -> Vector {
        Vector::new(self.text.len() as f32 * self.width(), self.size)
    }

    fn build(&mut self, mut builder: Builder) {
        let mut x = 0.0f32;
        for b in self.text.chars() {
            if b == ' ' {
                x += self.width();
                continue;
            }
            let index = if b >= 'a' && b <= 'z' {
                b as u32 - 'a' as u32 + 26
            } else if b >= 'A' && b <= 'Z' {
                b as u32 - 'A' as u32
            } else if b >= '0' && b <= '9' {
                b as u32 - '0' as u32 + 52
            } else {
                64
            };

            builder.draw_glyph(Vector::new(x, 0.0), Vector::new(x + self.width(), self.size), index);
            x += self.width();
        }
    }

    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }

    fn changed(&self) -> bool {
        false
    }
}

impl IntoComponent for &str {
    type Component = Text;

    fn into_component(self) -> Self::Component {
        Text::new(self.to_string())
    }
}