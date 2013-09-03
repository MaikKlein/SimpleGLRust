extern mod gl;

pub trait Buffer {
  fn set_buffer_data<T>(&self,
                       
                       vertices: &[T],
                       usage: gl::types::GLenum);
  fn bind(&self);
  fn get_handle(&self) -> gl::types::GLuint;
}
pub struct VertexBuffer{
    handle: ::handle::Handle
}
impl Buffer for VertexBuffer {
    fn set_buffer_data<T>(&self,
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
    fn get_handle(&self) -> gl::types::GLuint {
        self.handle.get()
    }
    fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 
                       self.handle.get());
    }
}
impl Drop for VertexBuffer {
  fn drop(&self) {
    unsafe {gl::DeleteBuffers(1, 
                              &self.get_handle())
    };
  }
  
}
impl VertexBuffer {
  pub fn new<T>(vertices: &[T],
                usage: gl::types::GLenum) -> VertexBuffer {
        let handle: gl::types::GLuint = 0;
        unsafe{gl::GenBuffers( 1,&handle )};
        let vbo = VertexBuffer{handle: ::handle::Handle::new(handle)};
        vbo.set_buffer_data(vertices,usage);
        vbo

  }
}
pub struct IndexBuffer{
    handle: ::handle::Handle
}
impl Buffer for IndexBuffer {
    fn set_buffer_data<T>(&self,
                              vertices: &[T],
                              usage: gl::types::GLenum) {
        match vertices.len(){
            0 => fail!(~"zero vertices"),
            _ => {
                //self.vbo.bind();
                self.bind();
                unsafe{gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_len(vertices) as gl::types::GLsizeiptr,
                                       ::std::cast::transmute(&vertices[0]),   
                                       usage)}}
        };
    }
    fn get_handle(&self) -> gl::types::GLuint {
        self.handle.get()
    }
    fn bind(&self) {
        //self.vbo.bind();
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 
                       self.handle.get());
    }
}
impl Drop for IndexBuffer {
  fn drop(&self) {
    unsafe {gl::DeleteBuffers(1, 
                              &self.get_handle())
    };
  }
  
}
impl IndexBuffer {
  pub fn new<T>(vertices: &[T],
             usage: gl::types::GLenum) -> IndexBuffer {
        let handle: gl::types::GLuint = 0;
        unsafe{gl::GenBuffers( 1,&handle )};
        let ibo = IndexBuffer{handle: ::handle::Handle::new(handle)};
        ibo.set_buffer_data(vertices,usage);
        ibo
  }
}

fn byte_len<T>(v: &[T]) 
                -> uint { 
    v.len() * ::std::sys::nonzero_size_of::<T>() 
}
