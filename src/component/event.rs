use core::position::Vector;
pub use glutin::event::MouseButton;
pub use glutin::event::KeyboardInput;

#[derive(Copy, Clone, Debug)]
pub enum Event{
    Mouse(Vector, MouseEvent),
    KeyBoard(KeyboardInput),
    Char(char),
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseEvent{
    Moved,
    Dragged,
    Pressed(MouseButton),
    Relased(MouseButton),
    Enter,
    Exit,
}
