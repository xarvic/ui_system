use native;
use native::component::*;
use native::process::{new_window, window};
use native::component::component::IntoComponent;
use core::color::RED;

fn main(){
    new_window(
        window(
            collumn()
                .child(
                    button("Test Text")
                        .onclick(||println!("clicked!"))
                ).child(
                    button("Test Text")
                        .onclick(||println!("clicked!"))
                ).child(
                button("Test Text".into_component())
                    .onclick(||println!("clicked!"))
            ).child(
                text_field("")
            )
        ).title("Test App")
    );
}