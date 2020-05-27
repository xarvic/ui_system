use native;
use native::process::WindowConstructor;
use native::component::*;
use native::process::{new_window, window};

fn main(){
    new_window(
        window(
            button("Test Component")
                .onclick(||println!("clicked!"))
        ).title("Test App")
    );
}