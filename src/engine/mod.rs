use glutin::event::{Event, WindowEvent, StartCause};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use glium::Display;

use crate::renderer::Renderer;
use crate::component::button;
use glutin::platform::unix::x11::EventLoopProxy;

pub struct WindowConstructor{

}

enum EngineEvent{
    NewWindow(WindowConstructor),

}

static mut event_proxy: Option<EventLoopProxy<EngineEvent>> = None;


pub fn run(first_window: Option<WindowConstructor>) {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap_or_else(|er|panic!("could not create the display!"));

    let mut renderer = Renderer::new(&display);

    let mut main_component = Box::new(button());

    event_loop.run(move|event, evl, control|{
        let mut render = false;
        match event {
            Event::WindowEvent {window_id, event} => {
                println!("window!");
                match event {
                    WindowEvent::Resized(_) => {render = true;},
                    WindowEvent::Moved(_) => {},
                    WindowEvent::Destroyed | WindowEvent::CloseRequested => {*control = ControlFlow::Exit},
                    _ => {}
                }
            },
            Event::NewEvents(StartCause::Init) => {

                println!("started!");
                *control = ControlFlow::Wait;
                render = true;
            }
            Event::UserEvent(event) => {
                println!("custom!");
            },
            _ => {},
        }
        if(render) {
            renderer.render_screen(&mut *main_component, display.draw());
        }
    });
}