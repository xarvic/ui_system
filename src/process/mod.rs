use crate::process::engine::Engine;
use crate::component::component::Component;
use glutin::event::Event;
use glutin::event_loop::{EventLoop, EventLoopProxy, ControlFlow};

mod engine;
mod command;
mod window;

pub struct WindowConstructor{
    main_component: Box<dyn Component>,
    titel: String,

}

impl WindowConstructor{
    pub fn new(titel: &str, main_component: impl Component + 'static) -> WindowConstructor{
        WindowConstructor{
            titel: titel.to_string(),
            main_component: Box::new(main_component),
        }
    }
}

pub enum EngineCommand {
    NewWindow(WindowConstructor),
    StateUpdate,
}

static mut event_proxy: Option<EventLoopProxy<EngineCommand>> = None;

fn run(first_window: Option<WindowConstructor>) {
    let event_loop = EventLoop::with_user_event();

    let mut engine = Engine::create(first_window, &event_loop)
        .expect("Could not create the Engine!");

    event_loop.run(move|event, _evl, control|{
        //When we get an event we poll all remaining

        match event {
            Event::WindowEvent { window_id, event } => {
                engine.handleWindowEvent(event, window_id);
                *control = ControlFlow::Poll;
            },
            Event::NewEvents(_cause) => {
                //Triggert when all events are processed
                *control = ControlFlow::Wait;

                engine.update_needed();
            }
            Event::UserEvent(command) => {
                engine.handleEngineCommand(command);
                *control = ControlFlow::Poll;
            }
            Event::RedrawRequested(id) => {
                engine.update(id, true);
                *control = ControlFlow::Poll;
            },
            _ => {},
        }
    });
}

pub fn new_window(constructor: WindowConstructor){
    run(Some(constructor));
}