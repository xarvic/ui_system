use crate::renderer::Builder;
use core::position::Vector;
use std::ops::{Deref, DerefMut};
use crate::component::event::Event;

pub trait Component{
    fn size(&self) -> Vector;
    fn pref_size(&self) -> Vector;
    fn build(&mut self, builder: Builder);
    fn handle_event(&mut self, event: Event) -> bool;
    fn changed(&self) -> bool;
}

impl<T: Component> Component for Box<T> {
    fn size(&self) -> Vector {
        self.deref().size()
    }

    fn pref_size(&self) -> Vector {
        self.deref().pref_size()
    }

    fn build(&mut self, buffer: Builder) {
        self.deref_mut().build(buffer)
    }

    fn handle_event(&mut self, event: Event) -> bool {
        self.deref_mut().handle_event(event)
    }

    fn changed(&self) -> bool {
        self.deref().changed()
    }
}

pub trait IntoComponent {
    type Component: Component;

    fn into(self) -> Self::Component;
}

impl<T> IntoComponent for T
where T: Component{
    type Component = Self;

    fn into(self) -> Self::Component {
        self
    }
}

