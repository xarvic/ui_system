pub use button::{Button, button};
pub use collum::{Collum, collumn};
pub use empty::Empty;
pub use row::{Row, row};
pub use text::{Text, text_field, TextField};
pub use component::{Component, IntoComponent};
use crate::renderer::style::Style;
use crate::renderer::Builder;
use crate::pool_tree::*;
use crate::core::Vector;
use crate::event::Event;

mod component;

mod button;
mod empty;
mod text;
mod row;
mod collum;

pub struct NewComponent {
    pub style: Option<Style>,
    changed: bool,
    pub size: Vector,
}

impl NewComponent {
    pub fn empty() -> Self {
        NewComponent{
            style: None,
            changed: false,
            size: Vector::null(),
        }
    }
    pub fn draw(mut self: NodeMut<Self>, mut builder: Builder) {
        let mut inner_size = self.size.clone();
        if let Some(ref style) = self.style {
            println!("apply css");
            builder = style.render(builder, &mut inner_size);
        }
        println!("draw component!");
        self.changed = false;
        self.childs_mut().map(|child|child.draw(builder.child_builder(Vector::null())));
    }

    pub fn handle_event(mut self: NodeMut<Self>, event: Event) -> bool {
        if let Some(ref mut style) = self.style {
            self.changed |= style.apply_event(event);
        }
        self.changed
    }
    pub fn has_changed(&self) -> bool {
        self.changed
    }
}