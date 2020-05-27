use crate::process::{WindowConstructor, EngineCommand};

use glutin::event::WindowEvent;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use glium::Display;

use crate::renderer::Renderer;
use std::collections::HashMap;
use glutin::window::WindowId;
use crate::process::window::ManagedWindow;
use glium::backend::Facade;
use std::time::Instant;

pub struct Engine{
    renderer: Renderer,
    windows: HashMap<WindowId, ManagedWindow>,
    last_sync: Instant,
}

impl Engine{
    pub fn create(constructor: Option<WindowConstructor>, event_loop: &EventLoop<EngineCommand>) -> Result<Engine, String> {
        let mut windows = HashMap::new();

        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();

        let context = if let Some(constructor) = constructor {
            let wb = wb.with_title(constructor.titel);

            let display = Display::new(wb, cb, &event_loop)
                .map_err(|err|err.to_string())?;

            let context = display.get_context().clone();

            let win = ManagedWindow::new(display, constructor.main_component);

            windows.insert(win.1, win.0);

            context
        } else {
            todo!("implement headless build for context");
            //cb.build_headless(&event_loop, PhysicalSize::new(0, 0))
            //    .map_err(|err|"could not create headless opengl context! ")?
        };

        Ok(Engine{windows, renderer: Renderer::new(&context), last_sync: Instant::now()})
    }
    pub fn handleEngineCommand(&mut self, _command: EngineCommand){

    }
    pub fn handleWindowEvent(&mut self, event: WindowEvent, id: WindowId){
        if let Some(window) = self.windows.get_mut(&id) {
            window.handle_event(event);
        }
    }
    ///
    ///
    pub fn update_needed(&mut self){
        for window in self.windows.iter_mut() {
            window.1.update(false, &mut self.renderer);
        }
    }
    /// updates all components of window corresponding to id
    /// and draws the window if needed
    ///
    /// force_redraw draws the window even if nothing changed
    pub fn update(&mut self, _id: WindowId, _force_redraw: bool){

    }
}