extern mod gl;

struct Shader{
    handle: ::handle::Handle
}
impl Shader {
    pub fn new(source: ~str,
           shader_type: gl::types::GLenum) 
           -> Result<Shader,~str> {
        let handle = gl::CreateShader(shader_type);
        do source.with_c_str |ptr|{
            unsafe {gl::ShaderSource(handle, 1, &ptr, ::std::ptr::null())};
        }
        gl::CompileShader(handle);
        

        let status = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &status);
             
            if status != (gl::TRUE as gl::types::GLint) {
                let len = 0;
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &len);
                let buf = ::std::vec::from_elem(len as uint, 0u8);
                gl::GetShaderInfoLog(handle, len, ::std::ptr::null(),
                ::std::vec::raw::to_ptr(buf) as *gl::types::GLchar);
                let s :~str= ::std::str::from_bytes(buf);
                let s = fmt!("\nFILE_NAME failed to compile:\n%?",s);
                Err(s)
            } 
            else {
                Ok(Shader{handle: ::handle::Handle::new(handle)})
            }
        }
    }
    pub fn new_vs(source: ~str) 
                  -> Result<Shader,~str>{
        Shader::new(source,gl::VERTEX_SHADER)   
    }
    pub fn new_fs(source: ~str) 
                  -> Result<Shader,~str>{
        Shader::new(source,gl::FRAGMENT_SHADER)   
    }
    pub fn new_cs(source: ~str) 
                  -> Result<Shader,~str>{
        Shader::new(source,gl::FRAGMENT_SHADER)   
    }
    pub fn new_gs(source: ~str) 
                  -> Result<Shader,~str>{
        Shader::new(source,gl::FRAGMENT_SHADER)   
    }
    pub fn new_ts(source: ~str) 
                  -> Result<Shader,~str>{
        Shader::new(source,gl::FRAGMENT_SHADER)   
    }
}

impl Drop for Shader {
    fn drop(&self) {
        gl::DeleteShader(self.handle.get());
    }
}
