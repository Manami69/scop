use std::ffi::{CStr, CString};
pub mod mathlib;
use mathlib::classes::{matrix::Matrix, vector::Vector};
use mathlib::operations::other::*;
pub mod render_gl;
use image::io::Reader as ImageReader;
use render_gl::{program::*, shader::*, window::Window};
use std::env::Args;
use std::io;
use std::io::Cursor;
pub mod env;
pub mod obj_parser;

fn main() -> Result<(), io::Error> {
    // args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }
    let mut obj = obj_parser::Objf::new();
    obj.read_file(&args[1]);
    ////////////////////////////////////////////////////////
    let (mut event_pump, window, video_subsystem) = Window::new();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 1280, 720); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    ////// TEST
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                                  // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
    // load image, create texture and generate mipmaps
    let img = image::open("wall.jpg").unwrap();
    let data = img.to_rgb8().into_raw();
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const gl::types::GLvoid,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    /////
    shader_program.set_used();
    // let vertices: Vec<f32> = vec![
    //     // // 	x, 		y,  	z,   	r,	b,	g
    //     // 0.5, -0.5, 0.0, 1., 0., 0., 0., 0., // bottom right
    //     // -0.5, -0.5, 0.0, 0., 1., 0., 1., 0., // bottom let
    //     // 0.0, 0.5, 0.0, 0., 0., 1., 0.5,
    //     // 1., // top

    //         -0.5, -0.5, -0.5,  0.0, 0.0,
    //          0.5, -0.5, -0.5,  1.0, 0.0,
    //          0.5,  0.5, -0.5,  1.0, 1.0,
    //          0.5,  0.5, -0.5,  1.0, 1.0,
    //         -0.5,  0.5, -0.5,  0.0, 1.0,
    //         -0.5, -0.5, -0.5,  0.0, 0.0,

    //         -0.5, -0.5,  0.5,  0.0, 0.0,
    //          0.5, -0.5,  0.5,  1.0, 0.0,
    //          0.5,  0.5,  0.5,  1.0, 1.0,
    //          0.5,  0.5,  0.5,  1.0, 1.0,
    //         -0.5,  0.5,  0.5,  0.0, 1.0,
    //         -0.5, -0.5,  0.5,  0.0, 0.0,

    //         -0.5,  0.5,  0.5,  1.0, 0.0,
    //         -0.5,  0.5, -0.5,  1.0, 1.0,
    //         -0.5, -0.5, -0.5,  0.0, 1.0,
    //         -0.5, -0.5, -0.5,  0.0, 1.0,
    //         -0.5, -0.5,  0.5,  0.0, 0.0,
    //         -0.5,  0.5,  0.5,  1.0, 0.0,

    //          0.5,  0.5,  0.5,  1.0, 0.0,
    //          0.5,  0.5, -0.5,  1.0, 1.0,
    //          0.5, -0.5, -0.5,  0.0, 1.0,
    //          0.5, -0.5, -0.5,  0.0, 1.0,
    //          0.5, -0.5,  0.5,  0.0, 0.0,
    //          0.5,  0.5,  0.5,  1.0, 0.0,

    //         -0.5, -0.5, -0.5,  0.0, 1.0,
    //          0.5, -0.5, -0.5,  1.0, 1.0,
    //          0.5, -0.5,  0.5,  1.0, 0.0,
    //          0.5, -0.5,  0.5,  1.0, 0.0,
    //         -0.5, -0.5,  0.5,  0.0, 0.0,
    //         -0.5, -0.5, -0.5,  0.0, 1.0,

    //         -0.5,  0.5, -0.5,  0.0, 1.0,
    //          0.5,  0.5, -0.5,  1.0, 1.0,
    //          0.5,  0.5,  0.5,  1.0, 0.0,
    //          0.5,  0.5,  0.5,  1.0, 0.0,
    //         -0.5,  0.5,  0.5,  0.0, 0.0,
    //         -0.5,  0.5, -0.5,  0.0, 1.0

    // ];
    let vertices = obj.get_v();

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
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
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            4,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (4 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        // gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        // gl::VertexAttribPointer(
        //     1,         // index of the generic vertex attribute ("layout (location = 0)")
        //     3,         // the number of components per generic vertex attribute
        //     gl::FLOAT, // data type
        //     gl::FALSE, // normalized (int-to-float conversion)
        //     (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        //     (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        // );
        // gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        // gl::VertexAttribPointer(
        //     1,         // index of the generic vertex attribute ("layout (location = 0)")
        //     2,         // the number of components per generic vertex attribute
        //     gl::FLOAT, // data type
        //     gl::FALSE, // normalized (int-to-float conversion)
        //     (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        //     (4 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        // );
    }
    let perspective: Matrix<f32> = Matrix::projection(radian(60.), 1.77777776, 0.1, 100.);

    let transform_loc: gl::types::GLint;
    let persp_loc: gl::types::GLint;
    let camera_loc: gl::types::GLint;

    unsafe {
        let cname = std::ffi::CString::new("transform").expect("CString::new failed");
        transform_loc = gl::GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("perspective").expect("CString::new failed");
        persp_loc = gl::GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("camera").expect("CString::new failed");
        camera_loc = gl::GetUniformLocation(shader_program.id(), cname.as_ptr());
    }

    let mut petit_puto: f32 = 0.4;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::BindVertexArray(vao);

            petit_puto += 0.05;

            let rot: Matrix<f32> = Matrix::mat4().rotate(petit_puto, Vector::vec3(0., 1., 0.)); //Matrix::projection(1.0472, 1.7777776, 0.1, 1000.);

            let trans: Matrix<f32> =
                rot.translate(petit_puto.cos(), petit_puto.sin(), -2.0 + petit_puto.cos()); //Matrix::projection(1.0472, 1.7777776, 0.1, 1000.);
            let view: Matrix<f32> = Matrix::view(
                Vector::vec3(0., 0., 10.),
                Vector::vec3(0., 0., 0.),
                Vector::vec3(0., 1., 0.),
            );
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, trans.as_ptr());
            gl::UniformMatrix4fv(persp_loc, 1, gl::FALSE, perspective.as_ptr());
            gl::UniformMatrix4fv(camera_loc, 1, gl::FALSE, view.as_ptr());

            gl::DrawArrays(
                gl::TRIANGLES,             // mode
                0,                         // starting index in the enabled arrays
                vertices.len() as i32 / 4, // number of indices to be rendered
            );
        }
        window.gl_swap_window();
        // render window contents here
    }
    Ok(())
}
