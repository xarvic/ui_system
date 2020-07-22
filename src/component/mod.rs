pub use button::Button;
pub use text::{Text, TextField};
use crate::renderer::style::Style;
use crate::renderer::Builder;
use crate::pool_tree::*;
use crate::core::Vector;
use crate::event::Event;
pub use crate::component::layout::{Layout, PreferredSize, HBox, Alignment};

mod button;
mod text;
mod layout;
mod build;
mod widget;

pub enum Content {
    Empty,
    Text(Text),
    TextField(TextField),
    Button(Button),
    Slider(),
    Container(Box<dyn Layout>),

}

pub enum Focus{
    None,
    This,
    Child(u32),
}

pub struct NewComponent {
    pub pos: Vector,
    pub size: Vector,
    changed: bool,

    focus: Focus,

    pub style: Option<Style>,

    pub content: Content,
    pub preferred_content_size: PreferredSize,
}

impl NewComponent {
    pub fn empty() -> Self {
        NewComponent{
            style: None,
            content: Content::Empty,
            changed: false,
            focus: Focus::None,
            size: Vector::null(),
            pos: Vector::null(),
            preferred_content_size: PreferredSize::empty(),
        }
    }
    pub fn draw(mut self: NodeMut<Self>, mut builder: Builder) {
        if let Some(ref style) = self.style {
            style.render(builder.id(), self.size);
            builder.translate(style.shift());
        }

        self.this().draw_content(builder.id());
        self.changed = false;
        for child in self.childs_mut() {
            let pos = child.pos;
            child.draw(builder.translate(pos))
        }
    }

    pub fn handle_event(mut self: NodeTop<Self>, mut event: Event) {
        println!("Handle Event {}!", self.index());
        self.changed |= if let Some(ref mut style) = self.style {
            let changed = style.apply_event(event);
            event = event.shift(style.shift());
            changed
        } else { false };


        let mut handler = None;

        if let Event::Mouse(v, e) = event {
            for child in self.childs() {
                println!("searching: {}", child.index());
                if v.in_rect(child.pos, child.size) {
                    println!("OK");
                    handler = Some(child.index());
                }
            }
        }
        let mut this = if let Some(id) = handler {
            println!("child Handle");
            self.focus = Focus::Child(id as u32);
            match self.to_id(id) {
                Ok(child) => {
                    let size = child.size;
                    println!("child Event");
                    child.handle_event(event.shift(size));
                    return;
                }
                Err(mut this) => {
                    this.focus = Focus::This;
                    println!("This Event");
                    this
                }
            }
        } else {
            self.focus = Focus::This;
            self
        };

        let consumed = this.as_mut().handle_event_content(event);
        if consumed {
            this.set_changed();
        }
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
    fn draw_content(self: NodeMut<Self>, mut builder: Builder) {
        match self.content {
            Content::Empty => {},
            Content::Text(ref text) => {
                text.build(builder.id())
            },
            Content::TextField(ref field) => {
                field.build(builder.id())
            },
            Content::Button(_) => {},
            Content::Slider() => {},
            Content::Container(_) => {},
        }
    }
    pub fn size(&self) -> Vector {
        self.size
    }
    pub fn set_size(mut self: NodeMut<Self>, size: Vector) {
        println!("Set Size of {} to {:?}", self.index(), size);
        if self.size != size {
            self.size = size;
            self.layout();
        }
    }
    pub fn layout(mut self: NodeMut<Self>) {
        println!("Changed Layout of {}", self.index());
        let (this, mut childs) = self.split();
        if let Content::Container(ref mut layout) = this.content {
            layout.layout(childs, this.size);
        } else {
            for mut child in childs.childs_mut() {
                //No Layout -> each component gets resized to its
                let size = child.this().preferred_size();
                child.set_size(size.preferred);
            }
        }
    }
    /// Updates the Layout in the Node Hierarchic
    ///
    pub fn changed_size(mut self: NodeTop<Self>){
        println!("Changed Size of {}", self.index());
        let my_id = self.index();
        match self.to_parent() {
            Err(mut this) => {
                //Top Element -> size is fixed
                this.this().layout();
                this.set_changed();
            }
            Ok(mut parent) => {
                let (this, childs) = parent.split();
                if let Content::Container(ref mut layout) = this.content {
                    let pref = layout.preferred_size(childs);
                    if pref != parent.preferred_content_size {
                        //If the parent size changes
                        parent.preferred_content_size = pref;
                        parent.changed_size();
                        return;
                    }
                    parent.this().layout();
                    parent.set_changed();
                }
            }
        }
    }
    pub fn preferred_size(self: NodeMut<Self>) -> PreferredSize {
        let wrap = self.style.as_ref().map_or(Vector::null(), |s|s.size());
        self.pref_content_size().wrap(wrap)
    }
    pub fn pref_content_size(mut self: NodeMut<Self>) -> PreferredSize {
        let (t, c) = self.split();
        match &t.content {
            Content::Empty => PreferredSize::empty(),
            Content::Text(text) => {PreferredSize::fixed(text.get_pref_size())}
            Content::TextField(text_field) => {PreferredSize::fixed(text_field.get_pref_size())}
            Content::Button(button) => {PreferredSize::fixed(button.get_pref_size())}
            Content::Slider() => {PreferredSize::empty()}
            Content::Container(container) => {container.preferred_size(c)}
        }
    }
    pub fn set_changed(mut self: NodeTop<Self>) {
        self.changed = true;
        if let Ok(parent) = self.to_parent() {
            parent.set_changed();
        }
    }
}