use std::sync::mpsc::Sender;

use glutin::event_loop::EventLoopProxy;

use crate::process::command::EngineCommand;
use crate::process::windows::WindowConstructor;

pub struct Environment {
    engine: EventLoopProxy<EngineCommand>,
}

impl Environment {
    pub fn new(engine: EventLoopProxy<EngineCommand>) -> Self {
        Environment {
            engine
        }
    }

    pub fn open(&mut self, constructor: WindowConstructor) {
        self.engine.send_event(EngineCommand::OpenWindow(constructor));
    }
}