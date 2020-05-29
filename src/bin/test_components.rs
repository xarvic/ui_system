use native;
use native::component::*;
use native::window;

use std::thread;
use native::process::{init, WindowConstructor};

fn main() {
    init(|mut env| {
        println!("client!");
        env.open(window(
            collumn()
                .child(
                    row()
                        .child(button("Test"))
                        .child((10, 10))
                        .child(button("Test2"))
                ).child(
                text_field("hi")
            )
        ));
    });
}