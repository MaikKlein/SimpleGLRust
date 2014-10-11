extern crate gl;

pub struct Shader{
    pub handle: ::handle::Handle
}
impl Shader {
    pub fn new(name: String,source: String,
               shader_type: gl::types::GLenum)
               -> Result<Shader,String> {
                   let handle = gl::CreateShader(shader_type);
                   source.with_c_str( |ptr|{
                       unsafe {gl::ShaderSource(handle, 1, &ptr, ::std::ptr::null())};
                   });
                   gl::CompileShader(handle);

                   let mut status = gl::FALSE as gl::types::GLint;
                   unsafe {
                       gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status);
                       if status != (gl::TRUE as gl::types::GLint) {
                           let mut len = 0;
                           gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
                           let mut buf = Vec::from_elem(len as uint, 0u8);
                           gl::GetShaderInfoLog(handle, len, ::std::ptr::null_mut(),buf.as_mut_ptr() as *mut i8);
                           let s = String::from_utf8(buf);
                           let s = format!("\n {} failed to compile:\n {}",name,s);
                           Err(s)
                       }
                       else {
                           Ok(Shader{handle: ::handle::Handle::new(handle)})
                       }
                   }
               }
    pub fn new_vs(name: String,source: String)
                  -> Result<Shader,String>{
                      Shader::new(name,source,gl::VERTEX_SHADER)
                  }
    pub fn new_fs(name: String,source: String)
                  -> Result<Shader,String>{
                      Shader::new(name,source,gl::FRAGMENT_SHADER)
                  }
    pub fn new_cs(name: String,source: String)
                  -> Result<Shader,String>{
                      Shader::new(name,source,gl::FRAGMENT_SHADER)
                  }
    pub fn new_gs(name: String,source: String)
                  -> Result<Shader,String>{
                      Shader::new(name,source,gl::FRAGMENT_SHADER)
                  }
    pub fn new_ts(name: String,source: String)
                  -> Result<Shader,String>{
                      Shader::new(name,source,gl::FRAGMENT_SHADER)
                  }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::DeleteShader(self.handle.get());
    }
}
