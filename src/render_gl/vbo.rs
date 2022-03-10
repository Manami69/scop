
pub struct Vbo {
	id : gl::types::GLuint,
    gl: gl::Gl,
}

impl Vbo {
	pub fn new(gl: &gl::Gl)  -> Self{
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Self { id : vbo, gl:gl.clone()}
	}

    pub fn set_vertex(&self, vert : &Vec<f32>) {
        unsafe {self.gl.BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vert.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vert.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );}
    }

	pub fn bind(&self) {
		unsafe {self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id() as gl::types::GLuint);}
	}
	
    pub fn unbind(&self) {
		unsafe {self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);}
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
			self.gl.DeleteBuffers(1, &self.id)
        }
    }
}