use native;
use native::component::*;
use native::window;

fn main() {
    window(
        collumn()
            .child(
                row()
                    .child(button("Test"))
                    .child((10, 10))
                    .child(button("Test2"))
            ).child(
            text_field("hi")
        )
    )
        .title("Test App")
        .open();
}