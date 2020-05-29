use glium::{Display, Surface};
use glutin::ContextBuilder;
use glutin::event::{ElementState, Event, MouseButton, StartCause, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use glutin::window::WindowBuilder;
use std::time::{Instant, Duration};
use std::ops::Add;

fn main() {
    let mut ev_loop = EventLoop::new();

    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &ev_loop)
        .unwrap_or_else(|e| panic!("cant create Display: {:?}", e));

    let mut count: f32 = 0.0;
    let mut invisible: Option<Instant> = None;

    ev_loop.run(move |e, win, control| {
        *control = ControlFlow::Wait;

        if let Some(ref i) = invisible {
            if i.ge(&Instant::now()) {
                display.gl_window().window().set_visible(true);
            }
        }

        if let Event::WindowEvent {window_id: _, event} = e {
            //let mut frame = display.draw();
            count = (count + 0.001) % 1.0;
            println!("count: {}", count);

            //frame.clear_color(count * count, 1.0, (1.0 - count)*(1.0-count), 1.0);

            //frame.finish();
            if event == WindowEvent::CloseRequested {
                *control = ControlFlow::Exit;
            }
            if let WindowEvent::MouseInput {..} = event {
                println!("wait!");
                display.gl_window().window().set_visible(false);
                *control = ControlFlow::WaitUntil(Instant::now().add(Duration::from_secs(2)));
                invisible = Some(Instant::now().add(Duration::from_secs(2)));
            }
        }
    });
}