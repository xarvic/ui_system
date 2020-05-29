use crate::process::windows::WindowConstructor;
use std::sync::mpsc::Sender;
use crate::process::command::EngineCommand;

use glutin::event_loop::{EventLoopProxy};

pub struct Environment {
    engine: EventLoopProxy<EngineCommand>,
}

impl Environment {
    pub fn new(engine: EventLoopProxy<EngineCommand>) -> Self{
        Environment{
            engine
        }
    }

    pub fn open(&mut self, constructor: WindowConstructor) {
        self.engine.send_event(EngineCommand::OpenWindow(constructor));
    }
}