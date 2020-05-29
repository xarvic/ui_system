use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};

pub use windows::{ManagedWindow, window};

use crate::process::engine::Engine;
use crate::process::windows::WindowConstructor;

mod engine;
mod windows;
mod command;

fn run(first_window: Option<WindowConstructor>) {
    let event_loop = EventLoop::with_user_event();

    let mut engine = Engine::create(first_window, &event_loop)
        .expect("Could not create the Engine!");

    event_loop.run(move |event, _evl, control| {
        match event {
            Event::WindowEvent { window_id, event } => {
                engine.handle_window_event(event, window_id);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            }
            Event::NewEvents(_cause) => {
                //Triggert when all events are processed
                *control = ControlFlow::Wait;
                engine.update_needed();
            }
            Event::UserEvent(command) => {
                engine.handle_engine_command(command);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            }
            Event::RedrawRequested(id) => {
                engine.update(id, true);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            }
            _ => {}
        }
        if engine.empty() {
            //TODO: dont close app
            *control = ControlFlow::Exit;
        }
    });
}

pub fn new_window(constructor: WindowConstructor) {
    run(Some(constructor));
}