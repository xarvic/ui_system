use crate::component::component::Component;
use crate::renderer::Builder;
use crate::component::event::Event;
use core::position::Vector;

pub struct Collum{
    childs: Vec<Box<dyn Component>>,
    spacing: f32,
    border: f32,
    width: Option<f32>,
    size: Vector,
    changed: bool,
}

pub fn collumn() -> Collum {
    Collum::new()
}

impl Collum {
    pub fn new() -> Self{
        Collum{
            childs: Vec::new(),
            spacing: 10.0,
            border:10.0,
            width: None,
            size: Vector::new(20.0, 20.0),
            changed: false,
        }
    }
    pub fn child(mut self, child: impl Component + 'static) -> Self{
        self.size = Vector::new(self.size.x.max(child.size().x), self.size.y + 10.0 + child.size().y);
        self.childs.push(Box::new(child));
        self
    }
}

impl Component for Collum {
    fn size(&self) -> Vector {
        self.size
    }

    fn pref_size(&self) -> Vector {
        self.size
    }

    fn build(&mut self, mut builder: Builder) {
        println!("build!");
        let mut translate_y = 10.0;
        for child in self.childs.iter_mut() {
            child.build(builder.child_builder(Vector::new(10.0, translate_y)));
            translate_y += 10.0 + child.size().y;
        }
        self.changed = false;
    }

    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Mouse(pos, event) => {
                if pos.x >= 10.0 && pos.x <= self.size.x - 10.0 {
                    let mut translate_y = 10.0;

                    for child in self.childs.iter_mut() {
                        if pos.y > translate_y && pos.y < translate_y + child.size().y {
                            let child_change = child.handle_event(Event::Mouse(pos.xy(-10.0, -translate_y), event));
                            self.changed = self.changed || child_change;
                            return self.changed;
                        }

                        translate_y += 10.0 + child.size().y;
                    }
                }
            },
            Event::None => {},
        }
        false
    }

    fn changed(&self) -> bool {
        self.changed
    }
}