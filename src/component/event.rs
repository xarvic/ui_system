use core::position::Vector;
pub use glutin::event::MouseButton;

pub enum Event{
    Mouse(Vector, MouseEvent),
    None,
}

pub enum MouseEvent{
    Moved,
    Dragged,
    Pressed(MouseButton),
    Relased(MouseButton),
    Enter,
    Exit,
}
