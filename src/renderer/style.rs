use crate::renderer::Builder;
use std::rc::Rc;
use crate::core::{Color, Vector};
use crate::event::{Event, MouseEvent, MouseButton};

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
    pub fn apply(&self, mut builder: Builder, size: Vector) {
        if let Some(ref background) = self.background {
            match background {
                Background::Color(c) => builder.round_rect(Vector::null(), size, *c, self.border_radius),
            }
        }
        if let Some(ref parts) = self.parts {
            builder.border(Vector::null(), size, parts, self.border_radius);
        }
    }

    pub fn empty() -> Self {
        StyleSheet {
            border_radius: [0.0, 0.0, 0.0, 0.0],
            parts: None,
            background: None,
        }
    }

    pub fn background(&mut self, back: Background) -> &mut Self {
        self.background = Some(back);
        self
    }
    pub fn backgorund_color(&mut self, red: f32, green: f32, blue: f32) -> &mut Self{
        self.background(Background::Color(Color::new(red, green, blue, 1.0)));
        self
    }
    pub fn border_radius(&mut self, radius: f32) -> &mut Self {
        self.border_radius = [
            radius,
            radius,
            radius,
            radius,
        ];
        self
    }

    pub fn simple_border(&mut self, width: f32, radius: f32, color: Color) -> &mut Self {
        let part = BorderPart{width, color};

        self.parts = Some([
                part.clone(),
                part.clone(),
                part.clone(),
                part.clone()
            ]);
        self
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
    pub fn new(sheet: Rc<StyleCollection>) -> Self {
        Style{
            style: sheet,
            disabled: false,
            focused: false,
            hovered: false,
            clicked: false,
        }
    }
    pub fn current_sheet(&self) -> &StyleSheet {
        if self.disabled {
            &self.style.disabled
        } else if self.clicked {
            &self.style.clicked
        } else if self.hovered {
            &self.style.hovered
        } else if self.focused {
            &self.style.focused
        } else {
            &self.style.idle
        }
    }
    pub fn render(&self, builder: Builder, size: Vector) {
        let sheet = self.current_sheet();

        sheet.apply(builder, size)
    }
    /// changes the state focused, hovered, clicked according to the event
    ///
    /// returns if anything changed
    pub fn apply_event(&mut self, event: Event) -> bool {
        match event {
            Event::Mouse(_, event) => {
                match event {
                    MouseEvent::Pressed(b) => {
                        if b == MouseButton::Left && !self.clicked {
                            self.clicked = true;
                            return true;
                        }
                    },
                    MouseEvent::Relased(b) => {
                        if b == MouseButton::Left && self.clicked {
                            self.clicked = false;
                            return true;
                        }
                    },
                    MouseEvent::Enter => {
                        if !self.hovered {
                            self.hovered = true;
                            return true;
                        }
                    },
                    MouseEvent::Exit => {
                        if self.hovered || self.clicked {
                            self.hovered = false;
                            self.clicked = false;
                            return true;
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        return false;
    }

    pub fn shift(&self) -> Vector {
        if let Some(ref border) = self.current_sheet().parts {
            Vector::new(border[1].width + border[3].width, border[0].width + border[2].width)
        } else {
            Vector::null()
        }
    }

    pub fn size(&self) -> Vector {
        if let Some(ref border) = self.current_sheet().parts {
            Vector::new(border[3].width, border[0].width)
        } else {
            Vector::null()
        }
    }

}