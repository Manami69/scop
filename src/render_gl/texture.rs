pub struct Texture {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

impl Texture {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            // set the texture wrapping parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        Self {
            id: texture,
            gl: gl.clone(),
        }
    }

    pub fn new_cube(gl: &gl::Gl) -> Self {
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_CUBE_MAP, texture);

            gl.TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl.TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl.TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl.TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl.TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
        }
        Self {
            id: texture,
            gl: gl.clone(),
        }
    }
    /// all upcoming GL_TEXTURE_2D operations now have effect on this texture object
    pub fn bind(&self) {
        unsafe {
            self.gl
                .BindTexture(gl::TEXTURE_2D, self.id as gl::types::GLuint);
        }
    }

    pub fn bind_cube(&self) {
        unsafe {
            self.gl
                .BindTexture(gl::TEXTURE_CUBE_MAP, self.id as gl::types::GLuint);
        }
    }
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn load(&self, path: String) {
        self.bind();
        let img;
		match image::open(path) {
			Ok(open) => img = open,
			Err(e) => panic!("{} : {}", "IMAGE PARSING ERROR", e)

		}
        let data = img.to_rgb8().into_raw();
        unsafe {
            self.gl.TexImage2D(
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
            self.gl.GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn load_cube(&self, paths: Vec<String>) {
        if paths.len() != 6 {
            panic!("cubic texture must have 6 faces");
        }
        self.bind_cube();
        for (i, face) in paths.iter().enumerate() {
            let img = image::open(face).unwrap();
            let data = img.to_rgb8().into_raw();
            unsafe {
                self.gl.TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGB as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    &data[0] as *const u8 as *const gl::types::GLvoid,
                );
                self.gl.GenerateMipmap(gl::TEXTURE_CUBE_MAP);
            }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &self.id) }
    }
}
