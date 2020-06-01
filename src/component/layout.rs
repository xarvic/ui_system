use crate::core::Vector;

#[derive(Copy, Clone)]
pub struct PreferredSize{
    pub minimum: Vector,
    pub grow_x: bool,
    pub grow_y: bool,
}

impl PreferredSize {
    pub fn new(x: f32, y: f32, grow_x: bool, grow_y: bool) -> Self{
        PreferredSize{
            minimum: Vector::new(x, y),
            grow_x,
            grow_y,
        }
    }
}

/// Layout is responsible for changing the the size and position of the child components
///
pub trait Layout{
    /// This method is called when this components size or a child-component preferred size changed
    fn layout(&mut self) -> bool;

    ///calculates the preferred size for this component according to the children
    fn preferred_size(&self) -> PreferredSize;
}

#[derive(Copy, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
    Fill,
}

pub struct HBox {
    v_align: Alignment,
    size: Vector,
    preferred_size: PreferredSize,
}

impl HBox {
    pub fn new(v_align: Alignment) -> Self {
        HBox{
            v_align,
            size: Vector::new(0.0, 0.0),
            preferred_size: PreferredSize::new(0.0, 0.0, false, false)
        }
    }
    pub fn layout(&mut self) {

    }
}

