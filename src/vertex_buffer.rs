extern mod gl;


struct VertexBuffer{
    handle: ::handle::Handle
}

impl VertexBuffer {
    pub fn set_buffer_data<T>(&self,
                       
                       vertices: &[T],
                       usage: gl::types::GLenum) {
        match vertices.len(){
            0 => fail!(~"zero vertices"),
            _ => {self.bind();
                unsafe{gl::BufferData(gl::ARRAY_BUFFER, byte_len(vertices) as gl::types::GLsizeiptr,
                                       ::std::cast::transmute(&vertices[0]),   
                                       usage)}}
        };
    }
    pub fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 
                       self.handle.get());
    }
    pub fn new() -> VertexBuffer {
        let handle: gl::types::GLuint = 0;
        unsafe{gl::GenBuffers( 1,&handle )};
        VertexBuffer{handle: ::handle::Handle::new(handle)}
    }
}
impl Drop for VertexBuffer {
    fn drop(&self) {
        unsafe {gl::DeleteBuffers(1, 
                                  &self.handle.get())
        };
    }
}
fn byte_len<T>(v: &[T]) 
                -> uint { 
    v.len() * ::std::sys::nonzero_size_of::<T>() 
}
