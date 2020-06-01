use std::ops::DerefMut;

use glium::{Display, Surface};
use glutin::event::WindowEvent;
use glutin::window::WindowId;

use crate::component::NewComponent;
use crate::renderer::Renderer;
use crate::core::Vector;
use crate::state::StorageID;
use crate::pool_tree::PoolTree;

pub struct WindowConstructor {
    pub(crate) titel: Option<String>,
    pub(crate) close_handler: Option<Box<dyn FnMut() -> bool>>,
}

unsafe impl Send for WindowConstructor {}

impl WindowConstructor {
    pub fn new() -> WindowConstructor {
        WindowConstructor {
            titel: None,
            close_handler: None,
        }
    }
    pub fn title(mut self, title: &str) -> Self {
        self.titel = Some(title.to_string());
        self
    }
    pub fn on_close(mut self, handler: impl FnMut() -> bool + 'static) -> Self {
        self.close_handler = Some(Box::new(handler));
        self
    }
}

pub fn window() -> WindowConstructor {
    WindowConstructor::new()
}

pub struct ManagedWindow {
    display: Display,
    close_handler: Option<Box<dyn FnMut() -> bool>>,
    dpi: f32,
    components: PoolTree<NewComponent>,
    last_mouse_position: Vector,
    redraw: bool,
    closed: bool,
}

impl ManagedWindow {
    pub fn new(display: Display, constructor: WindowConstructor) -> Self {
        display.gl_window().window().set_visible(true);
        let dpi = display.gl_window().window().current_monitor().scale_factor() as f32;
        ManagedWindow {
            display,
            close_handler: constructor.close_handler,
            dpi,
            last_mouse_position: Vector::null(),
            redraw: true,
            closed: false,
            components: PoolTree::new(NewComponent::empty())
        }
    }
    pub fn into_inner(self) -> Display {
        self.display
    }
    pub fn handle_event(&mut self, event: WindowEvent) -> bool {
        match event {
            WindowEvent::Resized(_size) => {
                self.redraw = true;
            }
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
            }
            /*WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_position = Vector::new(position.x as f32, position.y as f32);

                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Moved));
            }
            WindowEvent::CursorEntered { .. } => {
                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Enter));
            }
            WindowEvent::CursorLeft { .. } => {
                self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Exit));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let glutin::event::ElementState::Pressed = state {
                    self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Pressed(button)));
                } else {
                    self.main_component.handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Relased(button)));
                }
            }
            WindowEvent::ReceivedCharacter(charac) => {
                self.main_component.handle_event(Event::Char(charac));
            }
            WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                self.main_component.handle_event(Event::KeyBoard(input));
            }*/

            _ => {}
        }
        false
    }
    pub fn state_change(&mut self, state_ids: &[StorageID]) {}

    pub fn id(&self) -> WindowId {
        self.display.gl_window().window().id()
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn update(&mut self, force_redraw: bool, renderer: &mut Renderer) {
        if self.redraw || force_redraw {
            renderer.render_screen(self.components.root(), self.display.draw());

            self.redraw = false;
        }
    }
}