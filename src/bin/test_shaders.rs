use native;
use native::component::*;
use native::process::{new_window, window};

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
                    button("Test Text")
                        .onclick(||println!("clicked!"))
                )
        ).title("Test App")
    );
}