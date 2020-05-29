use crate::renderer::Builder;
use core::position::Vector;
use std::ops::{Deref, DerefMut};
use crate::event::Event;

pub trait Component{
    fn get_size(&self) -> Vector;
    fn get_pref_size(&self) -> Vector;
    fn build(&mut self, builder: Builder);
    fn handle_event(&mut self, event: Event) -> bool;
    fn has_changed(&self) -> bool;
}

impl<T: Component> Component for Box<T> {
    fn get_size(&self) -> Vector {
        self.deref().get_size()
    }

    fn get_pref_size(&self) -> Vector {
        self.deref().get_pref_size()
    }

    fn build(&mut self, buffer: Builder) {
        self.deref_mut().build(buffer)
    }

    fn handle_event(&mut self, event: Event) -> bool {
        self.deref_mut().handle_event(event)
    }

    fn has_changed(&self) -> bool {
        self.deref().has_changed()
    }
}

pub trait IntoComponent {
    type Component: Component + 'static;

    fn into_component(self) -> Self::Component;
}

impl<T: 'static> IntoComponent for T
where T: Component{
    type Component = Self;

    fn into_component(self) -> Self::Component {
        self
    }
}

