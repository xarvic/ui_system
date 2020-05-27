use native;
use native::process::WindowConstructor;
use native::component::*;

fn main(){
    native::process::new_window(WindowConstructor::new("TestApp", button("Test Component")));
}