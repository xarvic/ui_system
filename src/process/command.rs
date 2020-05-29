use crate::state::StorageID;
use crate::process::windows::WindowConstructor;

use std::thread::spawn;
use std::sync::mpsc::{channel, Sender, Receiver};

use glutin::event_loop::{EventLoop, EventLoopProxy, ControlFlow};
use glutin::event::{Event, WindowEvent};
use glutin::window::WindowId;
use crate::process::engine::Engine;
use std::sync::Mutex;

pub enum EngineCommand {
    OpenWindow(WindowConstructor),
    StateChange(StorageID),
}