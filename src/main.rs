use glutin::{
    event::{
        Event,
        WindowEvent
    },
    event_loop::{
        EventLoop,
        ControlFlow
    },
    window::WindowBuilder,
};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("triangle")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let ctx = glutin::ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { ctx.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s));

    // Setting some OpenGL options and stuff
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // Running the event loOoOop
    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            }
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    })
}

