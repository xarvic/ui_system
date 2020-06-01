use crate::renderer::Builder;

use glutin::event::{VirtualKeyCode, ElementState};
use crate::event::Event;
use crate::core::*;
use crate::state::State;

pub struct Text{
    pub text: String,
    pub size: f32,
    pub color: Color,
}

impl Text {
    pub fn new(text: String) -> Text {
        Text{text, size: 20.0, color: BLACK}
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        (self.size * 36.0 / 70.0).floor()
    }

    #[inline(always)]
    pub fn get_pref_size(&self) -> Vector {
        Vector::new(self.text.len() as f32 * self.width(), self.size)
    }

    #[inline(always)]
    pub fn build(&self, builder: &mut Builder) {
        draw_string(builder, &self.text, self.color, Vector::null(), self.size);
    }
}

pub struct TextField {
    current: Text,
    state: State<String>,
    cursor: usize,
}

impl TextField {
    pub fn new(mut state: State<String>) -> TextField{
        //TODO: register
        let text = Text::new(state.load_anonymous().0);
        let cursor = text.text.len();
        TextField{
            current: text,
            state,
            cursor,
        }
    }

    pub fn get_pref_size(&self) -> Vector {
        self.current.get_pref_size().xy(6.0, 6.0)
    }

    pub fn build(&self, builder: &mut Builder) {
        builder.rect(Vector::new(self.cursor as f32 * self.current.width() as f32, 0.0),
                     Vector::new(self.cursor as f32 * self.current.width() as f32 + 1.0, self.current.size),
                     self.current.color);
        self.current.build(builder);
    }

    pub fn handle_event(&mut self, event: Event) -> bool {
        let mut edited = false;
        match event {
            Event::Mouse(_, _) => {},
            Event::KeyBoard(key) => {
                if key.state != ElementState::Pressed{
                    return false ;
                }
                if let Some(v) = key.virtual_keycode {
                    match v {
                        VirtualKeyCode::Delete => {
                            if  self.cursor < self.current.text.len() {
                                let c = self.cursor;
                                self.state.update(move |mut str| {str.remove(c); str});
                                self.current.text.remove(self.cursor);
                                edited = true;
                            }
                        }
                        VirtualKeyCode::Left => {
                            if self.cursor > 0 {
                                self.cursor -= 1;
                                edited = true;
                            }
                        }
                        VirtualKeyCode::Right => {
                            if self.cursor < self.current.text.len() {
                                self.cursor += 1;
                                edited = true;
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
                        self.current.text.remove(self.cursor - 1);
                        let c = self.cursor;
                        self.state.update(move |mut str|{str.remove(c - 1); str});
                        self.cursor -= 1;
                        edited = true;
                    }
                } else {
                    self.current.text.insert(self.cursor, c);
                    let cur = self.cursor;
                    self.state.update(move |mut str|{str.insert(cur, c); str});
                    self.cursor += 1;
                    edited = true;
                }
            },
            _ => {},
        }


        edited
    }
}

#[inline(always)]
fn draw_string(builder: &mut Builder, text: &str, color: Color, pos: Vector, size: f32){
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