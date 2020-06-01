pub use button::Button;
pub use text::{Text, TextField};
use crate::renderer::style::Style;
use crate::renderer::Builder;
use crate::pool_tree::*;
use crate::core::Vector;
use crate::event::Event;
use crate::component::layout::Layout;

mod button;
mod text;
mod layout;

pub enum Content{
    Empty,
    Text(Text),
    TextField(TextField),
    Button(Button),
    Slider(),
    Container(Box<dyn Layout>),

}

pub struct NewComponent {
    pub style: Option<Style>,
    pub content: Content,
    changed: bool,
    pub size: Vector,
}

impl NewComponent {
    pub fn empty() -> Self {
        NewComponent{
            style: None,
            content: Content::Empty,
            changed: false,
            size: Vector::null(),
        }
    }
    pub fn draw(mut self: NodeMut<Self>, mut builder: Builder) {
        let mut inner_size = self.size.clone();
        if let Some(ref style) = self.style {
            builder = style.render(builder, &mut inner_size);
        }
        self.this().draw_content(builder.child_builder(Vector::null()));
        self.changed = false;
        for child in self.childs_mut() {
            child.draw(builder.child_builder(Vector::null()))
        }
    }

    pub fn handle_event(mut self: NodeMut<Self>, event: Event) -> bool {
        if let Some(ref mut style) = self.style {
            self.changed |= style.apply_event(event);
        }
        let consumed = self.this().handle_event_content(event);

        if !consumed {
            let mut child_change = false;
            for mut child in self.childs_mut() {
                let consumed = child.handle_event(event);
                if consumed {
                    child_change = true;
                    break;
                }
            }
            self.changed |= child_change;
        } else {
            self.changed = true;
        }


        self.changed
    }
    #[inline(always)]
    pub fn has_changed(&self) -> bool {
        self.changed
    }
    #[inline(always)]
    fn handle_event_content(mut self: NodeMut<Self>, event: Event) -> bool {
        match self.content {
            Content::Empty => {false},
            Content::Text(_) => {false},
            Content::TextField(ref mut field) => {
                field.handle_event(event)
            },
            Content::Button(ref mut button) => {
                button.handle_event(event)
            },
            Content::Slider() => {false},
            Content::Container(_) => {false},
        }
    }
    #[inline(always)]
    fn draw_content<'a, 'b>(self: NodeMut<'b, Self>, mut builder: Builder<'a>) -> Builder<'a> {
        match self.content {
            Content::Empty => {},
            Content::Text(ref text) => {
                text.build(&mut builder)
            },
            Content::TextField(ref field) => {
                field.build(&mut builder)
            },
            Content::Button(_) => {},
            Content::Slider() => {},
            Content::Container(_) => {},
        }
        builder
    }
}