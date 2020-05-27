use crate::component::component::Component;
use glium::Display;
use glutin::window::WindowId;
use core::position::Vector;
use glutin::event::WindowEvent;
use crate::renderer::Renderer;
use crate::component::event::{Event, MouseEvent};
use std::ops::{DerefMut, Deref};
use crate::process::WindowConstructor;


pub struct ManagedWindow {
    display: Display,
    main_component: Box<dyn Component>,
    close_handler: Option<Box<dyn FnMut() -> bool>>,
    dpi: f32,
    last_mouse_position: Vector,
    redraw: bool,
    closed: bool,
}

impl ManagedWindow {
    pub fn new(display: Display, constructor: WindowConstructor) -> (ManagedWindow, WindowId){
        let dpi = display.gl_window().window().current_monitor().scale_factor() as f32;
        let id = display.gl_window().window().id();
        (ManagedWindow{display,
            main_component: constructor.main_component,
            close_handler: constructor.close_handler,
            dpi,
            last_mouse_position: Vector::null(),
            redraw: true,
            closed: false,
        },
         id)
    }
    pub fn handle_event(&mut self, event: WindowEvent) -> bool {
        match event {
            WindowEvent::Resized(_size) => {
                self.redraw = true;
            },
            WindowEvent::Destroyed | WindowEvent::CloseRequested => {
                let close = if let Some(ref mut handler) = self.close_handler {
                    handler.deref_mut()()
                } else {
                    true
                };
                if close {
                    self.display.gl_window().window().set_visible(false);
                    self.closed = true;
                }
            },
            WindowEvent::CursorMoved { position, ..} => {
                self.last_mouse_position = Vector::new(position.x as f32, position.y as f32);

                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Moved));
            }
            WindowEvent::CursorEntered { ..} => {
                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Enter));
            }
            WindowEvent::CursorLeft { .. } => {
                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Exit));
            }
            WindowEvent::MouseInput {state, button, ..} => {
                if let glutin::event::ElementState::Pressed = state {
                    self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Pressed(button)));
                } else {
                    self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Relased(button)));
                }
            }
            _ => {}
        }
        false
    }
    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn update(&mut self, force_redraw: bool, renderer: &mut Renderer){
        if self.main_component.changed() || self.redraw || force_redraw {

            renderer.render_screen(self.main_component.deref_mut(), self.display.draw());
            self.redraw = false;
        }
    }
}