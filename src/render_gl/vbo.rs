pub struct Vbo {
	id : gl::types::GLuint,
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
			gl::DeleteBuffers(1, &self.id)
        }
    }
}