use crate::core::Vector;
use crate::pool_tree::ChildsMut;
use crate::component::NewComponent;

#[derive(Copy, Clone, PartialEq)]
pub struct PreferredSize{
    pub preferred: Vector,
    /// None if the component is not growing horizonal
    /// Some(size) if the component can get shrinked size is the minimum size
    pub grow_x: Option<f32>,
    pub grow_y: Option<f32>,
}

impl PreferredSize {
    pub fn growing(size: Vector, grow_x: Option<f32>, grow_y: Option<f32>) -> Self{
        PreferredSize{
            preferred: size,
            grow_x,
            grow_y,
        }
    }
    pub fn fixed(size: Vector) -> Self{
    PreferredSize{
            preferred: size,
            grow_x: None,
            grow_y: None,
        }
    }

    pub fn empty() -> Self {
        Self::fixed(Vector::null())
    }
    #[inline]
    pub fn min_x(&self) -> f32 {
        self.grow_x.unwrap_or_else(||self.preferred.x)
    }
    #[inline]
    pub fn min_y(&self) -> f32 {
        self.grow_y.unwrap_or_else(||self.preferred.y)
    }
    #[inline]
    pub fn chained_y(&self, pref: PreferredSize) -> PreferredSize{
        PreferredSize::growing(
            Vector::new(
                self.preferred.x.max(pref.preferred.x),
                self.preferred.y + pref.preferred.y,
            ),
            self.grow_x
                .or(pref.grow_x)
                .map(|_|self.min_x().max(pref.min_x())),
            self.grow_y
                .or(pref.grow_y)
                .map(|_|self.min_y() + pref.min_y()),
        )
    }
    #[inline]
    pub fn chain_x(&self, pref: PreferredSize) -> PreferredSize{
        PreferredSize::growing(
            Vector::new(
                self.preferred.x + pref.preferred.x,
                self.preferred.y.max(pref.preferred.y),
            ),
            self.grow_x
                .or(pref.grow_x)
                .map(|_|self.min_x() + pref.min_x()),
            self.grow_y
                .or(pref.grow_y)
                .map(|_|self.min_y().max( pref.min_y())),
        )
    }
    #[inline]
    pub fn wrap(&self, wrap_size: Vector) -> PreferredSize{
        PreferredSize::growing(
            self.preferred + wrap_size,
            self.grow_x.map(|value| value + wrap_size.x),
            self.grow_y.map(|value| value + wrap_size.y),
        )
    }

    #[inline]
    pub fn pref(&self) -> Vector {
        self.preferred
    }
    #[inline]
    pub fn min(&self) -> Vector {
        Vector::new(self.min_x(), self.min_y())
    }
}

/// Layout is responsible for changing the the size and position of the child components
///
pub trait Layout{
    /// This method is called when this components size or a child-component preferred size changed
    fn layout(&mut self, childs: ChildsMut<NewComponent>, size: Vector) -> bool;

    ///calculates the preferred size for this component according to the children
    fn preferred_size(&self, childs: ChildsMut<NewComponent>) -> PreferredSize;
}

#[derive(Copy, Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct HBox {
    v_align: Alignment,
    preferred_size: PreferredSize,
}

impl HBox {
    pub fn new(v_align: Alignment) -> Self {
        HBox{
            v_align,
            preferred_size: PreferredSize::empty()
        }
    }
}

impl Layout for HBox {
    fn layout(&mut self, mut childs: ChildsMut<NewComponent>, size: Vector) -> bool {
        let mut start_x = 0f32;
        let pref_size = self.preferred_size(childs.id());

        //let size = 1_f32.min(0_f32.max((size.x - pref_size.min_x()) / (pref_size.pref().x - pref_size.min_x())));

        for mut child in childs.childs_mut() {
            let child_size = child.this().preferred_size().pref();
            println!("Child Size: {:?}", child_size);
            child.pos = Vector::new(start_x, 0.0);
            child.set_size(child_size);
            start_x += child_size.x;
        }
        false
    }

    fn preferred_size(&self, mut childs: ChildsMut<NewComponent>) -> PreferredSize {
        let mut pref_size = PreferredSize::empty();

        for element in childs.childs_mut() {
            pref_size = pref_size.chain_x(element.preferred_size());
        }

        pref_size
    }
}
