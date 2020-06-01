use crate::renderer::Builder;
use std::ops::DerefMut;
use crate::event::{Event, MouseEvent, MouseButton};
use crate::core::{Vector, Color};

pub struct Button {
    on_click: Option<Box<dyn FnMut()>>,
    pressed: bool,
}

impl Button{
    pub fn new() -> Button{
        Button{
            on_click: None,
            pressed: false,
        }
    }
    #[inline]
    pub fn onclick(mut self, handler: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
    pub fn get_pref_size(&self) -> Vector{
        Vector::null()
    }

    pub(crate) fn handle_event(&mut self, event: Event) -> bool {

        if let Event::Mouse(_, mouse_event) = event {
            match mouse_event {
                MouseEvent::Pressed(MouseButton::Left) => {
                    if !self.pressed {
                        self.pressed = true;
                        return true;
                    }
                },
                MouseEvent::Exit => {
                    if self.pressed {
                        self.pressed = false;
                        return true;
                    }
                },
                MouseEvent::Relased(MouseButton::Left) => {
                    if self.pressed {
                        //run handler
                        if let Some(ref mut handler) = self.on_click {
                            handler.deref_mut()();
                        }
                        self.pressed = false;
                        return true;
                    }
                },
                _ => {}
            }
        }
        false
    }
}