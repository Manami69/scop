use super::vbo::Vbo;

pub struct Vao {
    id : gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self {id : vao}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// Attrib
    /// 
    /// * `buffer` : VBO 
    /// 
	/// * `index` : location in vertex shader
    /// 
    /// * `nb_comp` : the number of data to read in the buffer for this attribution
	/// 
	/// * `stride` : the number of components per generic vertex attribute
	/// 
	/// * `offset` : offset of the first component
    /// 
    pub fn attrib(&self,buffer  : Vbo, index: gl::types::GLuint,nb_comp: gl::types::GLint,  stride: usize, offset: usize) {
        self.bind();
        buffer.bind();

        unsafe {
        gl::EnableVertexAttribArray(index); // this is "index (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            index,         // index of the generic vertex attribute ("index (location = 0)")
            nb_comp,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            if offset == 0 {std::ptr::null()} else {(offset * std::mem::size_of::<f32>()) as *const gl::types::GLvoid},                                     // offset of the first component
        );
    }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
			gl::DeleteVertexArrays(1, &self.id)
        }
    }
}