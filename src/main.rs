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
    let ctx = glutin::ContextBuilder::new()
        .with_vsync(false)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { ctx.make_current().unwrap() };

    gl::load_with(|s| windowed_context.get_proc_address(s));

    // Setting some OpenGL options and stuff
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // VAO
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
    }

    // VBO
    unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    }

    // Data
    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

    // Sending to buffers
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            core::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
            );
    }

    // Telling what we sent ?
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
            );
        gl::EnableVertexAttribArray(0);
    }

    // Actual vertex shader code
    const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

    // Actual fragment shader code
    const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
"#;

    unsafe {

        // Loading vertex shader to gpu
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);

        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
            );
        gl::CompileShader(vertex_shader);

        // Error check
        let mut success = 0;

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Loading fragment shader to gpu
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
            );
        gl::CompileShader(fragment_shader);

        // Error check
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Creating the Program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Error check
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        // Delete after detach
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        gl::UseProgram(shader_program);
    }

    // Running the event loOoOop
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            }
            Event::RedrawRequested(_) => {
                //println!("{:?}", event);
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    })
}

