fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("triangle")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    match glutin::ContextBuilder::new().build_windowed(wb, &el) {
            Ok(ctx) => {
                let windowed_context = unsafe { ctx.make_current().unwrap() };
                gl::load_with(|s| windowed_context.get_proc_address(s));
            },
            Err(e) => {eprintln!("{:?}", e);},
    }
}
