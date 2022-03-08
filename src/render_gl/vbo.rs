pub struct Vbo {
	id : gl::types::GLuint,
}

impl Vbo {
	pub fn new() -> Self{
		todo!()
	}

	pub fn use(&self) {
		gl::BindBuffer(gl::ARRAY_BUFFER, self.id() as gl::types::GLuint);
	}
	
	/// ATTRIB
	/// 
	/// * `layout` : location in vertex shader
	/// 
	/// * `stride` : the number of components per generic vertex attribute
	/// 
	/// * `offset` : offset of the first component
	pub fn attrib(layout: usize, stride: usize, offset: usize) {
		todo!();
	
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