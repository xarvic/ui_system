use crate::component::component::{IntoComponent, Component};
use crate::renderer::Builder;
use crate::event::Event;
use crate::core::Vector;

pub struct Empty(Vector);

impl Component for Empty {
    fn get_size(&self) -> Vector {
        self.0
    }

    fn get_pref_size(&self) -> Vector {
        self.0
    }

    fn build(&mut self, _builder: Builder) {}

    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }

    fn has_changed(&self) -> bool {
        false
    }
}

impl IntoComponent for () {
    type Component = Empty;

    fn into_component(self) -> Self::Component {
        Empty(Vector::null())
    }
}

impl IntoComponent for (i32, i32) {
    type Component = Empty;

    fn into_component(self) -> Self::Component {
        Empty(Vector::new(self.0 as f32, self.1 as f32))
    }
}