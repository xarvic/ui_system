#![allow(illegal_floating_point_literal_pattern)]
#![feature(arbitrary_self_types)]

#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;

pub use process::window;

pub(crate) mod renderer;
pub(crate) mod process;
pub mod component;
pub mod state;
pub mod event;
pub mod pool_tree;
pub mod core;

pub mod prelude {
    pub use crate::component::{
        button, Button,
        Collum,
        collumn, row,
        Row, Text,
        text_field, TextField,
        Component, IntoComponent};
    pub use crate::process::{init, window, WindowConstructor};
    pub use crate::state::{state, State, StorageID};
}

