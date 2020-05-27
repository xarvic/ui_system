use crate::component::component::{Component, IntoComponent};
use crate::renderer::Builder;
use crate::component::event::Event;
use core::position::Vector;

pub struct Text{
    text: String,
}

impl Text {
    fn new(text: String) -> Text {
        Text{text}
    }
}

impl Component for Text {
    fn size(&self) -> Vector {
        unimplemented!()
    }

    fn pref_size(&self) -> Vector {
        unimplemented!()
    }

    fn build(&mut self, builder: Builder) {
        unimplemented!()
    }

    fn handle_event(&mut self, event: Event) -> bool {
        unimplemented!()
    }

    fn changed(&self) -> bool {
        unimplemented!()
    }
}

impl IntoComponent for &str {
    type Component = Text;

    fn into(self) -> Self::Component {
        Text::new(self.to_string())
    }
}