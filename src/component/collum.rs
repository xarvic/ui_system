use crate::component::component::Component;
use crate::renderer::Builder;
use crate::event::{Event, MouseEvent};
use crate::core::Vector;

pub struct Collum{
    childs: Vec<Box<dyn Component>>,
    focused: Option<usize>,
    spacing: f32,
    border: f32,
    size: Vector,
    changed: bool,
}

pub fn collumn() -> Collum {
    Collum::new()
}

impl Collum {
    pub fn new() -> Self{
        Collum{
            focused: None,
            childs: Vec::new(),
            spacing: 10.0,
            border:10.0,
            size: Vector::new(20.0, 20.0),
            changed: false,
        }
    }
    pub fn child(mut self, child: impl Component + 'static) -> Self{
        self.size = Vector::new(self.size.x.max(child.get_size().x), self.size.y + self.spacing + child.get_size().y);
        self.childs.push(Box::new(child));
        self
    }
}

impl Component for Collum {
    fn get_size(&self) -> Vector {
        self.size
    }

    fn get_pref_size(&self) -> Vector {
        self.size
    }

    fn build(&mut self, mut builder: Builder) {
        let mut translate_y = self.border;
        for child in self.childs.iter_mut() {
            child.build(builder.child_builder(Vector::new(self.border, translate_y)));
            translate_y += self.spacing + child.get_size().y;
        }
        self.changed = false;
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Mouse(pos, event) => {
                if pos.x >= self.border && pos.x <= self.size.x - self.border {
                    let mut translate_y = self.border;

                    for (index, child) in self.childs.iter_mut().enumerate() {
                        if pos.y > translate_y && pos.y < translate_y + child.get_size().y {
                            let child_change = child.handle_event(Event::Mouse(pos.xy(-self.border, -translate_y), event));
                            self.changed = self.changed || child_change;
                            if let MouseEvent::Relased(_) | MouseEvent::Pressed(_) = event {
                                self.focused = Some(index);
                            }

                            return self.changed;
                        }

                        translate_y += self.spacing + child.get_size().y;
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