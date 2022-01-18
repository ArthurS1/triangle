use winit::{
    event::{
        WindowEvent,
        Event,
    },
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    window::Window,
};

fn main()
{
    let event_loop = EventLoop::new();

    match Window::new(&event_loop) {
        Ok(_) => {
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
                        eprintln!("Window should redraw.") 
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
