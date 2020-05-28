use native;
use native::component::*;
use native::process::window;
use native::component::component::IntoComponent;

fn main(){
    let mut click = 3;
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
            text_field("hi")
        )
    )
        .title("Test App")
        .open();
}