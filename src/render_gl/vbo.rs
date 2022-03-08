pub struct Vbo {
	id : gl::types::GLuint,
}

impl Vbo {
	pub fn new(vert : &Vec<f32>) -> Self{
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (vert.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vert.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }

        Self { id : vbo}
	}

	pub fn bind(&self) {
		unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, self.id() as gl::types::GLuint);}
	}
	
    pub fn unbind() {
		unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0);}
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
			gl::DeleteBuffers(1, &self.id)
        }
    }
}