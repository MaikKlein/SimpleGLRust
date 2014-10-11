extern crate gl;

struct ShaderProgram{
    handle: ::handle::Handle
}
impl ShaderProgram {
    pub fn link(&self) {
        gl::LinkProgram(self.handle.get());
    }
    pub fn use_program(&self) {
        gl::UseProgram(self.handle.get());
    }
    pub fn bind_frag_location(&self,
                              location: gl::types::GLuint,
                              name: String){
        name.with_c_str(
            |ptr|{
                unsafe {
                    gl::BindFragDataLocation(self.handle.get(),
                                             location,
                                             ptr)
                };
            }
            )
    }
    pub fn set_uniform1f(&self, name: String, value: gl::types::GLfloat) {
        name.with_c_str( |ptr|{
            unsafe {
                let location = gl::GetUniformLocation(self.handle.get(), ptr);
                gl::Uniform1f(location, value);
            }
        }
                          )
    }
    pub fn new(shaders: &[&::shader::Shader]) -> Result<ShaderProgram,String>{
        let sp = ShaderProgram {handle: ::handle::Handle::new(gl::CreateProgram())};
        for s in shaders.iter(){
            gl::AttachShader(sp.handle.get(), s.handle.get());
        }
        sp.link();

        let mut status = gl::FALSE as gl::types::GLint;
        unsafe { gl::GetProgramiv(sp.handle.get(), gl::LINK_STATUS, &mut status) };
        if status != (gl::TRUE as gl::types::GLint) {
            unsafe {
                let mut len: gl::types::GLint = 0;
                gl::GetProgramiv(sp.handle.get(), gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::from_elem(len as uint, 0u8);
                gl::GetProgramInfoLog(sp.handle.get(), len, ::std::ptr::null_mut(),
                                      buf.as_mut_ptr() as *mut i8);

                let s: String = String::from_utf8(buf).unwrap();
                return Err(s);
            }
        }
        else {
            Ok(sp)
        }
    }
    pub fn get_attrib_location(&self,
                               name:&str)
                               -> gl::types::GLuint{
                                   name.with_c_str( |ptr|{
                                       unsafe {gl::GetAttribLocation(self.handle.get(),
                                                                     ptr) as gl::types::GLuint
                                       }
                                   }
                                                     )
                               }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl::DeleteProgram(self.handle.get());
    }
}
