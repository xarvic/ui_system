use crate::component::component::Component;

pub struct Widget<State, Element: Component, Gen: FnMut(State) -> Element> {
    state: State,
    generator: Gen,
    inner: Element,
}

