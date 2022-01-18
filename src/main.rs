use winit::{
    event::{
        WindowEvent,
        Event,
    },
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    window::{
        Window,
    },
};

use raw_window_handle::{
    HasRawWindowHandle,
    RawWindowHandle,
};

use std::os::raw::c_void;

fn draw(handle: &RawWindowHandle)
{
    let gl = gl::load_with(|_| *handle as *const c_void);
}

fn main()
{
    let event_loop = EventLoop::new();

    match Window::new(&event_loop) {
        Ok(window) => {
            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id: _,
                    } => {
                        eprintln!("Closing window.");
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::RedrawRequested(_) => {
                        eprintln!("Window should redraw."); 
                        draw(&window.raw_window_handle());
                    }
                    _ => {
                        eprintln!("Unhandled event.");
                        ()
                    }
                }
            }); 
        },
        Err(os_error) => eprintln!("{}", os_error),
    }
    println!("Hello, world!");
}
