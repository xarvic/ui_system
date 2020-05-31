use ui_system::prelude::*;
use std::process::exit;

fn main() {
    init(|mut env| {
        println!("client!");
        let count = state(0);

        env.open(window(
            collumn()
                .child(
                    row()
                        .child(button("Test").onclick(move||{count.update(|v|v+1)}))
                        .child(button("Test2"))
                ).child(
                    text_field("hi")
                ).child(
                    button("test3")
                ).child(
                    button("test3")
                )
        ).on_close(||exit(0)));
    });
}