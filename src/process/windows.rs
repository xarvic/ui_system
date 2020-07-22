use std::ops::DerefMut;

use glium::{Display, Surface};
use glutin::event::WindowEvent;
use glutin::window::WindowId;

use crate::component::{NewComponent, Content, Text, TextField, HBox, Alignment};
use crate::renderer::Renderer;
use crate::core::{Vector, Color};
use crate::state::{StorageID, state};
use crate::pool_tree::{PoolTree, NodeTop};
use crate::renderer::style::{Style, StyleSheet, StyleCollection, Background};
use std::rc::Rc;
use crate::event::{Event, MouseEvent};

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

fn test_node(mut node: NodeTop<NewComponent>) {
    let mut sheet = StyleSheet::empty();
    //sheet.backgorund_color(1.0, 0.5, 0.0)
    //    .border_radius(20.0);


    let mut collection = StyleCollection::unchanged(sheet.clone());

    collection.hovered = sheet.backgorund_color(1.0, 0.2, 0.0).clone();
    collection.clicked = sheet.backgorund_color(1.0, 0.0, 0.1)
        .simple_border(5.0, 7.0, Color::new(1.0, 0.2, 0.2, 1.0)).clone();

    let collection = Rc::new(collection);
    let state = state("Test".to_owned());

    node.content = Content::Container(Box::new(HBox::new(Alignment::Center)));

    {
        let mut node = node.add_child(NewComponent::empty());
        node.style = Some(Style::new(collection.clone()));
        node.size = Vector::new(150.0, 50.0);
        node.content = Content::TextField(TextField::new(state.clone()));
    }
    {
        let mut node = node.add_child(NewComponent::empty());
        node.style = Some(Style::new(collection));
        node.size = Vector::new(150.0, 50.0);
        node.content = Content::TextField(TextField::new(state.clone()));
    }

    node.as_mut().layout();
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

        let mut tree = PoolTree::new(NewComponent::empty());

        test_node(tree.root_mut());

        ManagedWindow {
            display,
            close_handler: constructor.close_handler,
            dpi,
            last_mouse_position: Vector::null(),
            redraw: true,
            closed: false,
            components: tree
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
            WindowEvent::CursorMoved { position, .. } => {
                self.last_mouse_position = Vector::new(position.x as f32, position.y as f32);

                self.components.root_mut().handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Moved));
            }
            WindowEvent::CursorEntered { .. } => {
                self.components.root_mut().handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Enter));
            }
            WindowEvent::CursorLeft { .. } => {
                self.components.root_mut().handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Exit));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let glutin::event::ElementState::Pressed = state {
                    self.components.root_mut().handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Pressed(button)));
                } else {
                    self.components.root_mut().handle_event(Event::Mouse(self.last_mouse_position, MouseEvent::Relased(button)));
                }
            }
            WindowEvent::ReceivedCharacter(charac) => {
                self.components.root_mut().handle_event(Event::Char(charac));
            }
            WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                self.components.root_mut().handle_event(Event::KeyBoard(input));
            }

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


    //The update order is
    // - event triggert
    //   - component change registert vie changed flag
    //   - state changed
    //-------------Event Pipeline------------
    // - engine updates state
    //   - window changes affected widgets
    //   - window updates layout
    //   - redraw
    pub fn update(&mut self, force_redraw: bool, renderer: &mut Renderer) {
        if self.redraw || force_redraw || self.components.root().has_changed() {
            renderer.render_screen(self.components.root_mut().as_mut(), self.display.draw());

            self.redraw = false;
        }
    }
}