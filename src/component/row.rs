use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;
use crate::component::event::{Event, MouseEvent};
use core::position::Vector;

pub struct Row {
    childs: Vec<Box<dyn Component>>,
    focused: Option<usize>,
    spacing: f32,
    border: f32,
    size: Vector,
    changed: bool,
}

pub fn row() -> Row {
    Row::new()
}

impl Row {
    pub fn new() -> Self{
        Row {
            focused: None,
            childs: Vec::new(),
            spacing: 10.0,
            border:0.0,
            size: Vector::new(20.0, 20.0),
            changed: false,
        }
    }
    pub fn child(mut self, child: impl IntoComponent + 'static) -> Self{
        let child = child.into_component();
        self.size = Vector::new(self.size.x + self.spacing + child.get_size().x, self.size.y.max(child.get_size().y));
        self.childs.push(Box::new(child));
        self
    }
}

impl Component for Row {
    fn get_size(&self) -> Vector {
        self.size
    }

    fn get_pref_size(&self) -> Vector {
        self.size
    }

    fn build(&mut self, mut builder: Builder) {
        let mut translate_x = self.border;
        for child in self.childs.iter_mut() {
            child.build(builder.child_builder(Vector::new(translate_x, self.border)));
            translate_x += self.spacing + child.get_size().x;
        }
        self.changed = false;
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Mouse(pos, event) => {
                if pos.y >= self.border && pos.y <= self.size.x - self.border {
                    let mut translate_x = self.border;

                    for (index, child) in self.childs.iter_mut().enumerate() {
                        if pos.x > translate_x && pos.x < translate_x + child.get_size().x {
                            let child_change = child.handle_event(Event::Mouse(pos.xy( -translate_x, -self.border), event));
                            self.changed = self.changed || child_change;
                            if let MouseEvent::Relased(_) | MouseEvent::Pressed(_) = event {
                                self.focused = Some(index);
                            }

                            return self.changed;
                        }

                        translate_x += self.spacing + child.get_size().y;
                    }
                }
                if let MouseEvent::Relased(_) | MouseEvent::Pressed(_) = event {
                    self.focused = None;
                }
            },
            event => {
                if let Some(focused) = self.focused {
                    if let Some(child) = self.childs.get_mut(focused){
                        let child_change = child.handle_event(event);
                        self.changed = self.changed || child_change;
                    }
                }
            }
        }
        self.has_changed()
    }

    fn has_changed(&self) -> bool {
        self.changed
    }
}