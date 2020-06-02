use crate::core::Vector;
use crate::pool_tree::ChildsMut;
use crate::component::NewComponent;

#[derive(Copy, Clone)]
pub struct PreferredSize{
    pub preferred: Vector,
    /// None if the component is not growing horizonal
    /// Some(size) if the component can get shrinked size is the minimum size
    pub grow_x: Option<f32>,
    pub grow_y: Option<f32>,
}

impl PreferredSize {
    pub fn new(x: f32, y: f32, grow_x: Option<f32>, grow_y: Option<f32>) -> Self{
        PreferredSize{
            preferred: Vector::new(x, y),
            grow_x,
            grow_y,
        }
    }
    pub fn empty() -> Self {
        Self::new(0.0, 0.0, None, None)
    }
    #[inline(always)]
    pub fn min_x(&self) -> f32 {
        self.grow_x.unwrap_or_else(||self.preferred.x)
    }
    #[inline(always)]
    pub fn min_y(&self) -> f32 {
        self.grow_y.unwrap_or_else(||self.preferred.y)
    }
    #[inline(always)]
    pub fn grow_x(&self) -> bool {
        self.grow_x.is_some()
    }
    #[inline(always)]
    pub fn grow_y(&self) -> bool {
        self.grow_y.is_some()
    }
    pub fn paralel_x(&mut self, pref: PreferredSize) {
        self.preferred.x = self.preferred.x.max(pref.preferred.x);
        self.grow_x = if self.grow_x() || pref.grow_x() {
            Some(self.min_x().max( pref.min_x()))
        } else {
            None
        };
    }
    pub fn chain_x(&mut self, pref: PreferredSize) {
        self.preferred.x += pref.preferred.x;
        self.grow_x = if self.grow_x() || pref.grow_x() {
            Some(self.min_x() + pref.min_x())
        } else {
            None
        };
    }
    pub fn paralel_y(&mut self, pref: PreferredSize) {
        self.preferred.y = self.preferred.y.max(pref.preferred.y);
        self.grow_y = if self.grow_y() || pref.grow_y() {
            Some(self.min_y().max( pref.min_y()))
        } else {
            None
        };
    }
    pub fn chain_y(&mut self, pref: PreferredSize) {
        self.preferred.y += pref.preferred.y;
        self.grow_y = if self.grow_y() || pref.grow_y() {
            Some(self.min_y() + pref.min_y())
        } else {
            None
        };
    }
}

/// Layout is responsible for changing the the size and position of the child components
///
pub trait Layout{
    /// This method is called when this components size or a child-component preferred size changed
    fn layout(&mut self, childs: ChildsMut<NewComponent>) -> bool;

    ///calculates the preferred size for this component according to the children
    fn preferred_size(&self, childs: ChildsMut<NewComponent>) -> PreferredSize;
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
            preferred_size: PreferredSize::new(0.0, 0.0, None, None)
        }
    }
}

impl Layout for HBox {
    fn layout(&mut self, mut childs: ChildsMut<NewComponent>) -> bool{
        false
    }

    fn preferred_size(&self, mut childs: ChildsMut<NewComponent>) -> PreferredSize {
        let mut pref = PreferredSize::empty();

        for child in childs.childs_mut() {
            pref.paralel_y(child.preferred_size());
            pref.chain_x(child.preferred_size());
        }
        pref
    }
}

