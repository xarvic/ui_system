#![allow(illegal_floating_point_literal_pattern)]

#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;

pub use process::window;

pub mod renderer;
pub mod component;
pub mod process;
pub mod state;

