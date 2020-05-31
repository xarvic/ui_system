use std::collections::HashMap;
use std::rc::Rc;

use glium::backend::{Context, Facade};
use glium::Display;
use glutin::ContextBuilder;
use glutin::event::WindowEvent;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::window::WindowId;

use crate::process::command::EngineCommand;
use crate::process::windows::ManagedWindow;
use crate::renderer::Renderer;
use crate::state::{StateHandle, StorageID};

pub struct Engine {
    renderer: Renderer,
    windows: HashMap<WindowId, ManagedWindow>,
    unused_windows: Vec<Display>,
    states: HashMap<StorageID, StateHandle>,
    changed_states: Vec<StorageID>,
}

impl Engine {
    pub fn create(event_loop: &EventLoop<EngineCommand>) -> Result<Engine, String> {
        let windows = HashMap::new();

        let mut unused = Vec::new();

        for i in 0..3 {
            let wb = WindowBuilder::new();
            let cb = ContextBuilder::new();

            let display = Display::new(wb, cb, &event_loop)
                .map_err(|err| err.to_string())?;
            display.gl_window().window().set_visible(false);
            unused.push(display);
        }
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();

        let display = Display::new(wb, cb, &event_loop)
            .map_err(|err| err.to_string())?;

        let context = display.get_context().clone();

        display.gl_window().window().set_visible(false);

        unused.push(display);

        Engine::new(windows, unused, &context)
    }
    fn new(windows: HashMap<WindowId, ManagedWindow>, unused_windows: Vec<Display>, context: &Rc<Context>) -> Result<Engine, String> {
        Ok(Engine {
            windows,
            renderer: Renderer::new(context).map_err(|e| format!("cant create Renderer: {:?}", e))?,
            states: HashMap::new(),
            unused_windows,
            changed_states: Vec::new(),
        })
    }

    pub fn handle_engine_command(&mut self, event: EngineCommand) {
        println!("Got event!");
        match event {
            EngineCommand::OpenWindow(con) => {
                let window = if let Some(display) = self.unused_windows.pop() {
                    ManagedWindow::new(display, con)
                } else {
                    panic!("No windows!");
                };
                println!("create Window");
                self.windows.insert(window.id(), window);
            }
            EngineCommand::StateChange(id) => {
                println!("Update ID!");
                if !self.changed_states.contains(&id) {
                    self.changed_states.push(id);
                }
            }
        }
    }

    pub fn handle_window_event(&mut self, event: WindowEvent, id: WindowId) {
        let mut remove = false;
        if let Some(window) = self.windows.get_mut(&id) {
            window.handle_event(event);
            remove = window.closed();
        }
        if remove {
            println!("closed window!");
            if let Some(win) = self.windows.remove(&id) {
                println!("Recycle!");
                self.unused_windows.push(win.into_inner().0);
            }
        }
    }
    ///
    ///
    pub fn update_needed(&mut self) {
        let mut updated = Vec::new();

        while let Some(id) = self.changed_states.pop() {
            let update = if let Some(state) = self.states.get(&id) {
                //Updating states happens directly before drawing
                //therefore no inconsistent states can get displayed
                state.sync()
            } else {
                false
            };
            if update {
                updated.push(id);
            }
        }


        for window in self.windows.iter_mut() {
            window.1.state_change(&updated[..]);
            window.1.update(false, &mut self.renderer);
        }
    }
    /// updates all components of window corresponding to id
    /// and draws the window if needed
    ///
    /// force_redraw draws the window even if nothing changed
    pub fn update(&mut self, id: WindowId, force_redraw: bool) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.update(force_redraw, &mut self.renderer);
        }
    }

    pub fn empty(&self) -> bool {
        self.windows.len() == 0
    }
}