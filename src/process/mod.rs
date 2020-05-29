use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop};

pub use windows::{ManagedWindow, window, WindowConstructor};

use crate::process::engine::Engine;
use crate::process::environment::Environment;
use std::sync::mpsc::channel;
use std::thread::spawn;

mod engine;
mod windows;
mod command;
mod environment;

pub fn init(f: impl FnOnce(Environment) + Send + 'static) {

    let event_loop = EventLoop::with_user_event();

    let mut engine = Engine::create(&event_loop)
        .expect("Could not create the Engine!");

    let mut env = Environment::new(event_loop.create_proxy());
    spawn(||f(env));

    println!("START!!!!!!");

    event_loop.run(move |event, evl, control| {
        match event {
            Event::WindowEvent { window_id, event } => {
                engine.handle_window_event(event, window_id);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            }
            Event::UserEvent(command) => {
                engine.handle_engine_command(command);
            }
            Event::NewEvents(_cause) => {
                //Triggert when all events are processed
                *control = ControlFlow::Wait;
                engine.update_needed();
            }
            Event::RedrawRequested(id) => {
                engine.update(id, true);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            }
            _ => {}
        }
    });
}