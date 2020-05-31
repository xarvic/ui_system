use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;
use std::ops::DerefMut;
use crate::event::{Event, MouseEvent, MouseButton};
use crate::core::{Vector, Color};

pub struct Button {
    inner: Option<Box<dyn Component>>,
    on_click: Option<Box<dyn FnMut()>>,
    pressed: bool,
    changed: bool,
}

impl Button{
    pub fn new(inner: Box<dyn Component>) -> Button{
        Button{
            inner: Some(inner),
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
    fn get_size(&self) -> Vector{
        if let Some(ref inner) = self.inner {
            inner.get_size().xy(12.0, 12.0)
        } else {
            Vector::new(40.0, 40.0)
        }
    }

    fn get_pref_size(&self) -> Vector{
        if let Some(ref inner) = self.inner {
            inner.get_pref_size().xy(12.0, 12.0)
        } else {
            Vector::new(40.0, 40.0)
        }
    }

    fn build(&mut self, mut buffer: Builder) {
        let color = if self.pressed {
            Color::new(0.0, 0.0, 1.0, 1.0)
        } else {
            Color::new(0.5, 0.5, 1.0, 1.0)
        };

        buffer.round_rect(Vector::null(), self.get_size(), color, [10.0, 10.0, 10.0, 10.0]);
        buffer.simple_border(Vector::null(), self.get_size(), Color::new(0.0, 0.5, 1.0, 1.0), 1.0, 10.0);

        if let Some(ref mut inner) = self.inner {
            inner.deref_mut().build(buffer.child_builder(Vector::new(6.0, 6.0)));
        }
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

    fn has_changed(&self) -> bool {
        self.changed
    }
}

pub fn button(inner: impl IntoComponent) -> Button{
    Button::new(Box::new(inner.into_component()))
}