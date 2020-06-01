use crate::renderer::Builder;
use std::rc::Rc;
use crate::core::{Color, Vector};

#[derive(Clone)]
pub enum Background {
    Color(Color),
}

#[derive(Clone)]
pub struct BorderPart {
    pub width: f32,
    pub color: Color,
}

#[derive(Clone)]
pub struct StyleSheet {
    border_radius: [f32; 4],
    parts: Option<[BorderPart; 4]>,
    background: Option<Background>,
}

impl StyleSheet {
    pub fn apply<'a>(&self, mut builder: Builder<'a>, size: &mut Vector) -> Builder<'a> {
        if let Some(ref background) = self.background {
            match background {
                Background::Color(c) => builder.round_rect(Vector::null(), *size, *c, self.border_radius),
            }
        }
        if let Some(ref parts) = self.parts {
            builder.border(Vector::null(), *size, parts, self.border_radius);
            let builder = builder.child_builder(Vector::new(parts[0].width, parts[3].width));
            *size = size.xy(parts[0].width + parts[2].width, parts[1].width + parts[3].width);
        }

        builder
    }

    pub fn empty() -> Self {
        StyleSheet {
            border_radius: [0.0, 0.0, 0.0, 0.0],
            parts: None,
            background: None,
        }
    }

    pub fn background(&self, back: Background) -> Self {
        StyleSheet {
            background: Some(back),
            ..self.clone()
        }
    }
    pub fn simple_border(&self, width: f32, radius: f32, color: Color) -> Self {
        let part = BorderPart{width, color};
        StyleSheet{
            parts: Some([
                part.clone(),
                part.clone(),
                part.clone(),
                part.clone()
            ]),
            ..self.clone()
        }
    }
}

pub struct StyleCollection {
    pub idle: StyleSheet,
    pub focused: StyleSheet,
    pub hovered: StyleSheet,
    pub clicked: StyleSheet,
    pub disabled: StyleSheet,
}

impl StyleCollection {
    pub fn unchanged(sheet: StyleSheet) -> Self {
        StyleCollection {
            idle: sheet.clone(),
            focused: sheet.clone(),
            hovered: sheet.clone(),
            clicked: sheet.clone(),
            disabled: sheet.clone()
        }
    }
}

pub struct Style {
    style: Rc<StyleCollection>,

    disabled: bool,
    focused: bool,
    hovered: bool,
    clicked: bool,
}

impl Style {
    pub fn render<'a>(&self, builder: Builder<'a>, size: &mut Vector) -> Builder<'a> {
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
        sheet.apply(builder, size)
    }
}