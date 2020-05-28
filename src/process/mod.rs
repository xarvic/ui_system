use crate::process::engine::Engine;
use crate::component::component::Component;
use glutin::event::Event;
use glutin::event_loop::{EventLoop, ControlFlow};

mod engine;
mod command;
mod window;

pub struct WindowConstructor{
    main_component: Box<dyn Component>,
    titel: Option<String>,
    close_handler: Option<Box<dyn FnMut() -> bool>>
}

impl WindowConstructor{
    pub fn new(main_component: Box<dyn Component + 'static>) -> WindowConstructor{
        WindowConstructor{
            titel: None,
            main_component,
            close_handler: None,
        }
    }
    pub fn title(mut self, title: &str) -> Self{
        self.titel = Some(title.to_string());
        self
    }
    pub fn on_close(mut self, handler: impl FnMut() -> bool + 'static) -> Self{
        self.close_handler = Some(Box::new(handler));
        self
    }
    pub fn open(self) {
        new_window(self);
    }
}

pub fn window(main_component: impl Component + 'static) -> WindowConstructor {
    WindowConstructor::new(Box::new(main_component))
}

pub enum EngineCommand {
    NewWindow(WindowConstructor),
    StateUpdate,
}

fn run(first_window: Option<WindowConstructor>) {
    let event_loop = EventLoop::with_user_event();

    let mut engine = Engine::create(first_window, &event_loop)
        .expect("Could not create the Engine!");

    event_loop.run(move|event, _evl, control|{
        match event {
            Event::WindowEvent { window_id, event } => {
                engine.handle_window_event(event, window_id);

                //When we get an event we poll all remaining
                *control = ControlFlow::Poll;
            },
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
            },
            _ => {},
        }
        if engine.empty() {
            //TODO: dont close app
            *control = ControlFlow::Exit;
        }
    });
}

pub fn new_window(constructor: WindowConstructor){
    run(Some(constructor));
}