extern mod gl;

pub struct Handle{
    priv handle: gl::types::GLuint
}
impl Handle {
    pub fn get(&self) -> gl::types::GLuint {
        self.handle
    }
    pub fn new(handle: gl::types::GLuint) 
               -> Handle {
        Handle {handle: handle }
    }
}
