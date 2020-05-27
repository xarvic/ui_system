#![allow(illegal_floating_point_literal_pattern)]

pub mod renderer;
pub mod component;
pub mod process;

pub use process::window;

#[macro_use]
extern crate glium;