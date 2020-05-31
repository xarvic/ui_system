use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;

use glutin::event::{VirtualKeyCode, ElementState};
use crate::event::Event;
use crate::core::*;

pub struct Text{
    text: String,
    size: f32,
    color: Color,
}

impl Text {
    pub fn new(text: String) -> Text {
        Text{text, size: 20.0, color: BLACK}
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(&self) -> f32 {
        (self.size * 36.0 / 70.0).floor()
    }
}

impl Component for Text {
    fn get_size(&self) -> Vector {
        self.get_pref_size()
    }

    fn get_pref_size(&self) -> Vector {
        Vector::new(self.text.len() as f32 * self.width(), self.size)
    }

    fn build(&mut self, builder: Builder) {
        draw_string(builder, &self.text, self.color, Vector::null(), self.size);
    }

    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }

    fn has_changed(&self) -> bool {
        false
    }
}

pub struct TextField {
    inner: Text,
    cursor: usize,
    edited: bool,
}

impl TextField {
    pub fn new(text: Text) -> TextField{
        let cursor = text.text.len();
        TextField{
            inner: text,
            cursor,
            edited: false
        }
    }
}

impl Component for TextField {
    fn get_size(&self) -> Vector {
        self.inner.get_size().xy(6.0, 6.0)
    }

    fn get_pref_size(&self) -> Vector {
        self.inner.get_pref_size().xy(6.0, 6.0)
    }

    fn build(&mut self, mut builder: Builder) {
        builder.round_rect(Vector::null(),
                           self.get_size(),
                           Color::new(0.0, 0.0, 0.0, 0.2),
                           [3.0, 3.0, 3.0, 3.0]);
        builder.rect(Vector::new(self.cursor as f32 * self.inner.width() as f32 + 3.0, 3.0),
                     Vector::new(self.cursor as f32 * self.inner.width() as f32 + 4.0, self.inner.size + 3.0),
                     self.inner.color);
        self.inner.build(builder.child_builder(Vector::new(3.0, 3.0)));
        self.edited = false;
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Mouse(_, _) => {},
            Event::KeyBoard(key) => {
                if key.state != ElementState::Pressed{
                    return false ;
                }
                if let Some(v) = key.virtual_keycode {
                    match v {
                        VirtualKeyCode::Delete => {
                            if  self.cursor < self.inner.text.len() {
                                self.inner.text.remove(self.cursor);
                                self.edited = true;
                            }
                        }
                        VirtualKeyCode::Left => {
                            if self.cursor > 0 {
                                self.cursor -= 1;
                                self.edited = true;
                            }
                        }
                        VirtualKeyCode::Right => {
                            if self.cursor < self.inner.text.len() {
                                self.cursor += 1;
                                self.edited = true;
                            }
                        }
                        _ => {}
                    }
                }
            },
            Event::Char(c) => {
                if c as u32 == 127 {
                    //Entf code
                    return false;
                }
                if c as u32 == 8 {
                    //Remove code
                    if self.cursor > 0 {
                        //Remove previous
                        self.inner.text.remove(self.cursor - 1);
                        self.cursor -= 1;
                        self.edited = true;
                    }
                } else {
                    self.inner.text.insert(self.cursor, c);
                    self.cursor += 1;
                    self.edited = true;
                }
            },
            _ => {},
        }
        self.has_changed()
    }

    fn has_changed(&self) -> bool {
        self.edited
    }
}

pub fn text_field(initial: &str) -> TextField{
    TextField::new(initial.into_component())
}

fn draw_string(mut builder: Builder, text: &str, color: Color, pos: Vector, size: f32){
    let width = (size * 36.0 / 70.0).floor();

    let mut x = 0.0f32;
    for b in text.chars() {
        if b == ' ' {
            x += width;
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

        builder.glyph(pos.x(x), pos.xy(x + width, size), index, color);
        x += width;
    }
}

impl IntoComponent for &str {
    type Component = Text;

    fn into_component(self) -> Self::Component {
        Text::new(self.to_string())
    }
}