use crate::renderer::{CommandBuffer, Builder};
use core::position::Vector;

pub trait Component{
    fn size(&self) -> Vector;
    fn pref_size(&self) -> Vector;
    fn build(&mut self, buffer: Builder);
}