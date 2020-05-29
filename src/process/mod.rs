use glutin::event::Event;
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

pub use windows::{ManagedWindow, window, WindowConstructor};

use crate::process::engine::Engine;
use crate::process::environment::Environment;
use std::sync::mpsc::channel;
use std::thread::spawn;
use crate::process::command::EngineCommand;
use std::sync::Mutex;

pub mod engine;
mod windows;
pub mod command;
mod environment;

pub fn send_command(command: EngineCommand) {
    if let Some(ref queue) = *QUEUE.lock().unwrap() {
        queue.send_event(command);
    }
}

lazy_static!{
    static ref QUEUE: Mutex<Option<EventLoopProxy<EngineCommand>>> = {
        Mutex::new(None)
    };
}

pub fn init(f: impl FnOnce(Environment) + Send + 'static) {

    let event_loop = EventLoop::with_user_event();

    let mut engine = Engine::create(&event_loop)
        .expect("Could not create the Engine!");

    let mut env = Environment::new(event_loop.create_proxy());
    spawn(||f(env));

    *QUEUE.lock().unwrap() = Some(event_loop.create_proxy());

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