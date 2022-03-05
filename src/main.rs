extern crate gl;
extern crate sdl2; 

use std::ffi::CString;
pub mod render_gl;

fn main() {
    
    // init sdl2 and videosub
    let sdl = sdl2::init().unwrap(); 
    let video_subsystem = sdl.video().unwrap(); 
    
    // init opengl
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,1);
    
    // init window
    let window = video_subsystem
        .window("Pose App", 900, 700)
        .opengl() // add opengl flag
        .resizable() 
        .build()
        .unwrap();

    // init opengl context
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // init shaders & link shaders to program
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap(); 
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap(); 
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // create the triangle
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
         0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 
         0.0,  0.5, 0.0, 0.0, 0.0, 1.0
    ];
    
    // vbo buffer, upload data, binding it to the buffer
    let mut vbo: gl::types::GLuint = 0; 
    unsafe { gl::GenBuffers(1, &mut vbo);}
    unsafe { 
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); 
        gl::BufferData( 
            gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes 
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data 
            gl::STATIC_DRAW); // usage
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind buffer
    }

    let mut vao: gl::types::GLuint =0; 
    unsafe { gl::GenVertexArrays(1, &mut vao);}
    unsafe { 
        gl::BindVertexArray(vao); 
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); 
        gl::EnableVertexAttribArray(0); // layout location 0 in vertex shader
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT, 
            gl::FALSE,
            (6*std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null());

        gl::EnableVertexAttribArray(1); // layout location 1 in vertex shader
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT, 
            gl::FALSE,
            (6*std::mem::size_of::<f32>()) as gl::types::GLint,
            (3*std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind buffer
        gl::BindVertexArray(0);
    }
    
    unsafe {
        gl::Viewport(0,0,900,700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    
    // get application events
    let mut event_pump = sdl.event_pump().unwrap(); 
    // event loop 
    'main: loop {
        for event in event_pump.poll_iter(){
            // handle user input here 
            match event {
                sdl2::event::Event::Quit {..} => break 'main, 
                _ => {},
            }
        }
        // render window contents here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // use shader program (didn't call used bc it's a keyword) 
        shader_program.set_used();
        // switch shader, bind to vao and draw call
        unsafe{
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3);
        }
        window.gl_swap_window();
    }
}


