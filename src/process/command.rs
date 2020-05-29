use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread::spawn;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use glutin::window::WindowId;

use crate::process::engine::Engine;
use crate::process::windows::WindowConstructor;
use crate::state::StorageID;

pub enum EngineCommand {
    OpenWindow(WindowConstructor),
    StateChange(StorageID),
}