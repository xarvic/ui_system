use ui_system::prelude::*;
use std::process::exit;

fn main() {
    init(|mut env| {
        println!("client!");
        let count = state(0);

        env.open(window().on_close(||exit(0)));
    });
}