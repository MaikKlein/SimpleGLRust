extern mod gl;

struct VertexArray {
    handle: ::handle::Handle
}
impl VertexArray {
    pub fn new() -> VertexArray {
        let handle: gl::types::GLuint = 0;
        unsafe {gl::GenVertexArrays(1, &handle)};
        
        VertexArray{handle: ::handle::Handle::new(handle)}
        
    }
    pub fn bind(&self) {
        gl::BindVertexArray(self.handle.get());
    }
    pub fn bind_attrib(&self, location: gl::types::GLuint,
                   count: gl::types::GLint,
                   buffer: &::vertex_buffer::VertexBuffer) {
        buffer.bind();
        self.bind();
        gl::EnableVertexAttribArray(location);
        unsafe {gl::VertexAttribPointer(location, count, gl::FLOAT, gl::FALSE as u8, 0, ::std::ptr::null());}
        gl::EnableVertexAttribArray(0); // Disable our Vertex Array Object  
        gl::BindVertexArray(0); // Disable our Vertex Buffer Object 
    }
}

impl Drop for VertexArray {
    fn drop(&self) {
        unsafe{gl::DeleteVertexArrays(1,
                                      &self.handle.get());
        }
    }
}