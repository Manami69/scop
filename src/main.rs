use std::ffi::CString;
pub mod mathlib;
use mathlib::classes::{matrix::Matrix, vector::Vector};
use mathlib::operations::other::*;
pub mod render_gl;
use render_gl::{program::*, shader::*, texture::*, vao::Vao, vbo::Vbo, window::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::io;
pub mod env;
pub mod obj_parser;
use obj_parser::obj_file::Objfile;

#[warn(unused_variables)]
fn main() -> Result<(), io::Error> {
    // args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }
    let mut obj = Objfile::new();
    obj.read_file(&args[1]);
    ////////////////////////////////////////////////////////
    let (mut event_pump, window, video_subsystem) = Window::new();
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl.Viewport(0, 0, 1280, 720); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader = Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("shaders/triangle.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
    }

    shader_program.set_used();
    let vertices = obj.get_v();

    let vao = Vao::new(&gl);
    vao.bind();
    let vertices_buffer = Vbo::new(&gl);
    vertices_buffer.bind();
    vertices_buffer.set_vertex(&vertices);

    vao.attrib(0, 4, 4, 0);

    let perspective: Matrix<f32> = Matrix::projection(radian(60.), 1.77777776, 0.1, 100.);

    let transform_loc: gl::types::GLint;
    let persp_loc: gl::types::GLint;
    let camera_loc: gl::types::GLint;

    // TEXTURES MAPPING CUBE
    let mut texture: Texture = Texture::new_cube(&gl);
    // let faces = vec![
    //     "skybox/right.jpg".to_string(),
    //     "skybox/left.jpg".into(),
    //     "skybox/top.jpg".into(),
    //     "skybox/bottom.jpg".into(),
    //     "skybox/front.jpg".into(),
    //     "skybox/back.jpg".into(),
    // ];
	let faces = vec!["wall.jpg".to_string(); 6];
	texture.load_cube(faces);
	///////////
    unsafe {
        let cname = std::ffi::CString::new("transform").expect("CString::new failed");
        transform_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("perspective").expect("CString::new failed");
        persp_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("camera").expect("CString::new failed");
        camera_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

		let cname = std::ffi::CString::new("OurTexture").expect("CString::new failed");
        gl.Uniform1i(gl.GetUniformLocation(shader_program.id(), cname.as_ptr()), 0);
    }

    let mut trans_x: f32 = 0.;
    let mut trans_y: f32 = 0.;
    let mut trans_z: f32 = 0.;
    let mut cam_z: f32 = 0.;
    let mut turn_x: f32 = 0.;
    let mut turn_y = 0.;
    let mut turn_z = 0.;
    let mut poly: bool = false;
    let mut pas: f32 = 1.;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                // ZOOM CAMERA
                Event::MouseWheel { y: num, .. } => cam_z -= num as f32,
                // TRANSLATION COMMAND
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => trans_y += pas,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => trans_y += -pas,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => trans_x += -pas,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => trans_x += pas,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => trans_z += -pas,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => trans_z += pas,
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => pas += 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    if pas - 0.1 < 0.1 {
                        pas = 0.1
                    } else {
                        pas -= 0.1
                    }
                }
                // ROTATION COMMAND
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => turn_y += 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => turn_y += -0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => turn_x += -0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => turn_x += 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => turn_z += -0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => turn_z += 0.1,
                // ENABLE DISABLE POLYGON
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => poly = !poly,

                _ => {}
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            if poly {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
        vao.bind();
        unsafe {
			// bind texture
			gl.ActiveTexture(gl::TEXTURE0);
			texture.bind_cube();

            let mut model: Matrix<f32> = Matrix::mat4();
            model = model.rotate(turn_x, Vector::vec3(0., 1., 0.));
            model = model.rotate(turn_y, Vector::vec3(1., 0., 0.));
            model = model.rotate(turn_z, Vector::vec3(0., 0., 1.));

            model = model.translate(trans_x, trans_y, trans_z);
            let view: Matrix<f32> = Matrix::view(
                Vector::vec3(0., 0., cam_z),
                Vector::vec3(0., 0., 0.),
                Vector::vec3(0., 1., 0.),
            );
            gl.UniformMatrix4fv(transform_loc, 1, gl::FALSE, model.as_ptr());
            gl.UniformMatrix4fv(persp_loc, 1, gl::FALSE, perspective.as_ptr());
            gl.UniformMatrix4fv(camera_loc, 1, gl::FALSE, view.as_ptr());

            gl.DrawArrays(
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
