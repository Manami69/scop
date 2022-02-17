use std::ffi::{CStr, CString};
pub mod render_gl;
use render_gl::{shader::*, program::*, window::Window};
pub mod env;

fn main() {
    let (mut event_pump, window, video_subsystem) = Window::new();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

	let vert_shader = Shader::from_vert_source(
		&CString::new(include_str!("triangle.vert")).unwrap()
	).unwrap();
	
	let frag_shader = Shader::from_frag_source(
		&CString::new(include_str!("triangle.frag")).unwrap()
	).unwrap();

	let shader_program = Program::from_shaders(
		&[vert_shader, frag_shader]
	).unwrap();

	shader_program.set_used();
	let vertices: Vec<f32> = vec![
	// 	x, 		y,  	z,   	r,	b,	g
		0.5, 	-0.5, 	0.0, 	1., 0., 0.,   // bottom right
    	-0.5, 	-0.5, 	0.0, 	0.,	1.,	0.,  // bottom let
    	0.0,  	0.5, 	0.0, 	0.,	0.,	1.,   // top 

		0.5, 	0.8, 	0.0, 	1., 0., 0.,
		0.0, 	0.8, 	0.0, 	0.,	1.,	0.,
		0.0, 	0.5, 	0.0, 	0.,	0.,	1.,
        
	];

	let mut vbo: gl::types::GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
	}
	unsafe {
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER, // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW, // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	}
	let mut vao: gl::types::GLuint = 0;
	unsafe {
		gl::GenVertexArrays(2, &mut vao);
	}
	unsafe {
		gl::BindVertexArray(vao);
		
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			0, // index of the generic vertex attribute ("layout (location = 0)")
			3, // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			std::ptr::null() // offset of the first component
		);
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			1, // index of the generic vertex attribute ("layout (location = 0)")
			3, // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			(3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
		);
	}
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
		unsafe {
		    gl::DrawArrays(
		        gl::TRIANGLES, // mode
		        0, // starting index in the enabled arrays
		        vertices.len() as i32 / 3 // number of indices to be rendered
		    );
		}
        window.gl_swap_window();
        // render window contents here
    }
}
