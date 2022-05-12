use std::collections::HashMap;
use std::ffi::CString;
pub mod mathlib;
use env::ScopOption;
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
    if args.len() < 2 {
        print!(
            "\x1b[93mYou must launch the program with the obj file name as an arg
	cargo run [obj file path] [Options...]\n\x1b[0m
Options : 
	/sky : show the skybox
	/concave : check if the model has a concave polygon and panic! if so
	/coplanar : check if the model has a non-coplanar polygon and panic! if so
	/t texture_path : use a specific texture
bisous 😘\n"
        );
        return Ok(());
    }
    ////////////////////////////////////////////////////////
    let mut opt = ScopOption::new();

    if args.len() > 2 {
        opt.fill_options(args[2..].to_vec());
    }
    let (mut event_pump, window, video_subsystem) = Window::new();
    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl.Viewport(0, 0, 1280, 720); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    let mut obj = Objfile::new(&gl);
    obj.read_file(&args[1], &opt);
    // TODO: FOUTRE DANS LA CLASSE SHADER

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

    vao.attrib(0, 4, 15, 0); // vertices
    vao.attrib(1, 3, 15, 4); // rand color
    vao.attrib(2, 3, 15, 7); // default color
    vao.attrib(3, 2, 15, 10); // texture mapping
	vao.attrib(4, 3, 15, 12); // normal mapping

    // set skybox
    // let vert_skybox_shader = Shader::from_vert_source(
    //     &gl,
    //     &CString::new(include_str!("shaders/skybox.vert")).unwrap(),
    // )
    // .unwrap();

    // let frag_skybox_shader = Shader::from_frag_source(
    //     &gl,
    //     &CString::new(include_str!("shaders/skybox.frag")).unwrap(),
    // )
    // .unwrap();

    // let skybox_program =
    //     Program::from_shaders(&gl, &[vert_skybox_shader, frag_skybox_shader]).unwrap();
    // skybox_program.set_used();
    // let skybox_vao = Vao::new(&gl);
    // skybox_vao.bind();
    // let skybox_buffer = Vbo::new(&gl);
    // skybox_buffer.bind();
    // skybox_buffer.set_vertex(&Vec::from(env::SKYBOX_VERTICES));

    // skybox_vao.attrib(0, 3, 3, 0);

    /////
    let perspective: Matrix<f32> = Matrix::projection(radian(60.), 1.77777776, 0.1, 100.);

    let transform_loc: gl::types::GLint;
    let persp_loc: gl::types::GLint;
    let camera_loc: gl::types::GLint;
    let text_index: gl::types::GLint; // index de la texture sur laquelle on est
    let opacity: gl::types::GLint; // opacity of the next texture
    let texture1: gl::types::GLint; // custom texture
    let texture2: gl::types::GLint; // object texture
	let lighting: gl::types::GLint; // light position

    let uniform_pos_text: gl::types::GLint;

    let pos_text: Texture = Texture::new(&gl);
    pos_text.load("Ressources/Textures/large_qpupier.png".to_string());
    // let skybox_persp: gl::types::GLint;
    // let skybox_camera: gl::types::GLint;

    // TEXTURES MAPPING CUBE
    // let texture: Texture = Texture::new_cube(&gl);
    // let faces = vec![
    //     "skybox/right.jpg".to_string(),
    //     "skybox/left.jpg".into(),
    //     "skybox/top.jpg".into(),
    //     "skybox/bottom.jpg".into(),
    //     "skybox/front.jpg".into(),
    //     "skybox/back.jpg".into(),
    // ];
    // //let faces = vec!["wall.jpg".to_string(); 6];
    // texture.load_cube(faces);
    ///////////
    unsafe {
        let cname = std::ffi::CString::new("transform").expect("CString::new failed");
        transform_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("perspective").expect("CString::new failed");
        persp_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());
        // skybox_persp = gl.GetUniformLocation(skybox_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("camera").expect("CString::new failed");
        camera_loc = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());
        // skybox_camera = gl.GetUniformLocation(skybox_program.id(), cname.as_ptr());

        // let cname = std::ffi::CString::new("skybox").expect("CString::new failed");
        // gl.Uniform1i(
        //     gl.GetUniformLocation(skybox_program.id(), cname.as_ptr()),
        //     0,
        // );
        let cname = std::ffi::CString::new("texture_position").expect("CString::new failed");
        uniform_pos_text = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());
        let cname = std::ffi::CString::new("indextext").expect("CString::new failed");
        text_index = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());
        let cname = std::ffi::CString::new("opacity").expect("CString::new failed");
        opacity = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());


        let cname = std::ffi::CString::new("texture1").expect("CString::new failed");
        texture1 = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

        let cname = std::ffi::CString::new("texture2").expect("CString::new failed");
        texture2 = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());
		
		let cname = std::ffi::CString::new("lightDir").expect("CString::new failed");
        lighting = gl.GetUniformLocation(shader_program.id(), cname.as_ptr());

    }

    let mut m: env::ModelEvent = env::ModelEvent::new();

    'main: loop {
        if m.keys.get("W").is_some() {
            m.trans.z += 0.1;
        }
        if m.keys.get("S").is_some() {
            m.trans.z -= 0.1;
        }
        if m.keys.get("A").is_some() {
            m.trans.x += 0.1;
        }
        if m.keys.get("D").is_some() {
            m.trans.x -= 0.1;
        }
        if m.keys.get("F").is_some() {
            m.trans.y -= 0.1;
        }
        if m.keys.get("R").is_some() {
            m.trans.y -= 0.1;
        }
        if m.keys.get("Up").is_some() {
            m.turn.y += 0.1;
        }
        if m.keys.get("Down").is_some() {
            m.turn.y -= 0.1;
        }
        if m.keys.get("Right").is_some() {
            m.turn.x += 0.1;
        }
        if m.keys.get("Left").is_some() {
            m.turn.x -= 0.1;
        }
        if m.keys.get("Q").is_some() {
            m.turn.z -= 0.1;
        }
        if m.keys.get("E").is_some() {
            m.turn.z += 0.1;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                // ZOOM CAMERA
                Event::MouseWheel { y: num, .. } => m.cam_z -= num as f32,
                // TRANSLATION COMMAND
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    m.poly = !m.poly;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    if m.next_text == 0. {
                        m.next_text = 0.05;
                    }
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(key) => {
                            m.keys.insert(key.to_string(), true);
                        }
                        None => {}
                    };
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(key) => {
                            m.keys.remove(&(key.to_string()));
                        }
                        None => {}
                    };
                }
                _ => {}
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.DepthRange(0., 1.);
            if m.poly {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
        unsafe {
            let mut model: Matrix<f32> = Matrix::mat4();
            model = model.translate(-obj.mid.x, -obj.mid.y, -obj.mid.z);
            model = model.rotate(m.turn.x, Vector::vec3(0., 1., 0.));
            model = model.rotate(m.turn.y, Vector::vec3(1., 0., 0.));
            model = model.rotate(m.turn.z, Vector::vec3(0., 0., 1.));
            model = model.translate(obj.mid.x, obj.mid.y, obj.mid.z);

            model = model.translate(m.trans.x, m.trans.y, m.trans.z);
            // SKYBOX
            // gl.DepthMask(gl::FALSE);
            // skybox_program.set_used();
            // skybox_vao.bind();
            // position camera, position + vecteur front , up
            // let view = Matrix::view(
            //     Vector::vec3(0., 0., 0.),
            //     Vector::vec3(0., 0., -1.),
            //     Vector::vec3(0., 1., 0.),
            // );
            // gl.UniformMatrix4fv(skybox_persp, 1, gl::FALSE, perspective.as_ptr());
            // gl.UniformMatrix4fv(skybox_camera, 1, gl::FALSE, view.as_ptr());
            // bind texture
            // gl.ActiveTexture(gl::TEXTURE0);
            // texture.bind_cube();
            // gl.DrawArrays(gl::TRIANGLES, 0, 36);
            gl.DepthMask(gl::TRUE);
            let view: Matrix<f32> = Matrix::view(
                Vector::vec3(0., 00., m.cam_z),
                Vector::vec3(obj.mid.x, obj.mid.y, -obj.mid.z),
                Vector::vec3(0., 1., 0.),
            );
            shader_program.set_used();
            vao.bind();
            gl.Uniform1i(text_index, m.text_i);
            gl.Uniform1f(opacity, m.next_text);
            if m.next_text != 0. {
                m.next_text += 0.05;
                if m.next_text >= 1.0 {
                    m.text_i = (m.text_i + 1) % env::TEXT_MAX;
                    m.next_text = 0.;
                    //eprintln!("COUCOU {}", next_text);
                }
            }
			gl.Uniform1i(texture1, 0);
            gl.ActiveTexture(gl::TEXTURE0);
            pos_text.bind();
            gl.UniformMatrix4fv(transform_loc, 1, gl::FALSE, model.as_ptr());
            gl.UniformMatrix4fv(persp_loc, 1, gl::FALSE, perspective.as_ptr());
            gl.UniformMatrix4fv(camera_loc, 1, gl::FALSE, view.as_ptr());
			gl.Uniform3f(lighting, 2., -3., 5.);
            // TODO: dessiner par mtl
			for (i, val) in obj.tex.clone().into_iter().enumerate() {
				if val.show == false {continue ;}
				
				gl.Uniform1i(texture2, 1);
				gl.ActiveTexture(gl::TEXTURE1);
				match val.text_map {
					Some(tex) => {
						obj.textures.get(&tex).unwrap().bind();
						//println!("GLOUGLOU {}", &tex);
					},
					None => {  
						gl.BindTexture(gl::TEXTURE_2D, 0)
					}
				}
				
				gl.DrawArrays(
                gl::TRIANGLES,             // mode
                val.start,                         // starting index in the enabled arrays
                val.end - val.start , // number of indices to be rendered 
            );
		}
           
        }
        window.gl_swap_window();
        // render window contents here
    }
    Ok(())
}
