use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;
use core::position::Vector;
use core::color::Color;
use crate::component::event::{Event, MouseEvent, MouseButton};
use std::ops::DerefMut;

pub struct Button {
    inner: Option<Box<dyn Component>>,
    on_click: Option<Box<dyn FnMut()>>,
    pressed: bool,
    changed: bool,
}

impl Button{
    pub fn new() -> Button{
        Button{
            inner: None,
            on_click: None,
            pressed: false,
            changed: false,
        }
    }
    #[inline]
    pub fn onclick(mut self, handler: impl FnMut() + 'static) -> Self {
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
        let color = if self.pressed {
            Color::new(0.0, 0.0, 1.0, 1.0)
        } else {
            Color::new(0.5, 0.5, 1.0, 1.0)
        };

        buffer.draw_round_rect(Vector::null(), self.size(), color, [10.0, 10.0, 10.0, 10.0]);
        buffer.draw_simple_border(Vector::null(), self.size(), Color::new(0.0, 0.5, 1.0, 1.0), 3.0, 10.0);
        self.changed = false;
    }

    fn handle_event(&mut self, event: Event) -> bool {

        if let Event::Mouse(_, mouse_event) = event {
            match mouse_event {
                MouseEvent::Pressed(MouseButton::Left) => {
                    if !self.pressed {
                        self.pressed = true;
                        self.changed = true;
                    }
                },
                MouseEvent::Exit => {
                    if self.pressed {
                        self.pressed = false;
                        self.changed = true;
                    }
                },
                MouseEvent::Relased(MouseButton::Left) => {
                    if self.pressed {
                        //run handler
                        if let Some(ref mut handler) = self.on_click {
                            handler.deref_mut()();
                        }
                        self.pressed = false;
                        self.changed = true;
                    }
                },
                _ => {}
            }
        }
        self.changed
    }

    fn changed(&self) -> bool {
        self.changed
    }
}

pub fn button(inner: impl IntoComponent) -> Button{
    Button::new()
}