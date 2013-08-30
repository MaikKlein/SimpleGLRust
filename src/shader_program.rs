extern mod gl;

struct ShaderProgram<'self>{
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
                              name: ~str){
        do name.with_c_str |ptr|{
            unsafe {
                gl::BindFragDataLocation(self.handle.get(), 
                                         location, 
                                         ptr)
            };
        }
    }
    pub fn new(shaders: &[&::shader::Shader]) -> Result<ShaderProgram,~str>{
        
        let sp = ShaderProgram {handle: ::handle::Handle::new(gl::CreateProgram())};
        for s in shaders.iter(){
            gl::AttachShader(sp.handle.get(), s.handle.get());
        }
        sp.link();

        let status = gl::FALSE as gl::types::GLint;
        unsafe { gl::GetProgramiv(sp.handle.get(), gl::LINK_STATUS, &status) };
        if status != (gl::TRUE as gl::types::GLint) {
            unsafe {
                let len: gl::types::GLint = 0;
                gl::GetProgramiv(sp.handle.get(), gl::INFO_LOG_LENGTH, &len);
                 
                let buf = ::std::vec::from_elem(len as uint, 0u8);
                gl::GetProgramInfoLog(sp.handle.get(), len, ::std::ptr::null(),
                ::std::vec::raw::to_ptr(buf) as *gl::types::GLchar);
                 
                return Err(::std::str::raw::from_bytes(buf));
            }
        }
        else {
            Ok(sp)
        }
    }
    pub fn get_attrib_location(&self,
                               name:&str) 
                               -> gl::types::GLuint{
            do name.with_c_str |ptr|{
            unsafe {gl::GetAttribLocation(self.handle.get(), 
                                          ptr) as gl::types::GLuint
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&self) {
        gl::DeleteProgram(self.handle.get());
    }
}