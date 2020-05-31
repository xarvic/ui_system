use crate::renderer::Builder;
use std::rc::Rc;
use crate::core::{Color, Vector};

pub enum Background {
    Color(Color),
}

pub struct BorderPart {
    pub width: f32,
    pub color: Color,
}

struct StyleSheet{
    border_radius: [f32; 4],
    parts: Option<[BorderPart; 4]>,
    background: Option<Background>,
}

impl StyleSheet{
    pub fn apply<'a>(&self, mut builder: Builder<'a>, size: &mut Vector) -> Builder<'a> {
        if let Some(background) = self.background {
            match background {
                Color(c) => builder.round_rect(Vector::null(), *size, c, self.border_radius),
            }
        }
        if let Some(ref parts) = self.parts {
            builder.border(parts, self.border_radius);
            builder = builder.child_builder(Vector::new(parts[0].width, parts[3].width));
            *size = size.xy(parts[0].width + parts[2].width, parts[1].width + parts[3].width);
        }


        builder
    }
}

struct StyleCollection{
pub idle: StyleSheet,
pub focused: StyleSheet,
pub hovered: StyleSheet,
pub clicked: StyleSheet,
pub disabled: StyleSheet,
}

struct Style {
style: Rc<StyleCollection>,

disabled: bool,
focused: bool,
hovered: bool,
clicked: bool,
}

impl Style {
pub fn render<'a>(&mut self, builder: Builder<'a>) -> Builder<'a> {
let sheet = if self.disabled {
&self.style.disabled
} else if self.clicked {
&self.style.clicked
} else if self.hovered {
&self.style.hovered
} else if self.focused {
&self.style.focused
} else {
&self.style.idle
};
sheet.apply(builder)
}
}
}