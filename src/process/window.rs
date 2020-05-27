use crate::component::component::Component;
use glium::Display;
use glutin::window::WindowId;
use core::position::Vector;
use glutin::event::WindowEvent;
use crate::renderer::Renderer;
use crate::component::event::{Event, MouseEvent};
use std::ops::DerefMut;


pub struct ManagedWindow {
    display: Display,
    main_component: Box<dyn Component>,
    dpi: f32,
    last_mouse_position: Vector,
    redraw: bool,
}

impl ManagedWindow {
    pub fn new(display: Display, main_component: Box<dyn Component>) -> (ManagedWindow, WindowId){
        let dpi = display.gl_window().window().current_monitor().scale_factor() as f32;
        let id = display.gl_window().window().id();
        (ManagedWindow{display,
            main_component,
            dpi,
            last_mouse_position: Vector::null(),
            redraw: true,
        },
         id)
    }
    pub fn handle_event(&mut self, event: WindowEvent) -> bool {
        match event {
            WindowEvent::Resized(_size) => {
                self.redraw = true;
            },
            WindowEvent::Destroyed | WindowEvent::CloseRequested => {
                unimplemented!();
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

    pub fn update(&mut self, force_redraw: bool, renderer: &mut Renderer){
        if self.main_component.changed() || self.redraw || force_redraw {

            renderer.render_screen(self.main_component.deref_mut(), self.display.draw());
            self.redraw = false;
        }
    }
}