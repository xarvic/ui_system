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

mod component;

mod button;
mod empty;
mod text;
mod row;
mod collum;

pub struct NewComponent {
    style: Option<Style>,
    changed: bool,
    size: Vector,
}

impl NewComponent {
    pub fn empty() -> Self {
        NewComponent{
            style: None,
            changed: false,
            size: Vector::null(),
        }
    }
    pub fn draw(self: Node<Self>, mut builder: Builder) {
        let mut inner_size = self.size.clone();
        if let Some(ref style) = self.style {
            builder = style.render(builder, &mut inner_size);
        }

        self.childs().map(|child|child.draw(builder.child_builder(Vector::null())));
    }
}