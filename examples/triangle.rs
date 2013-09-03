extern mod gl;
extern mod glfw;
extern mod sgl;
use sgl::buffer::*;
use sgl::vertex_array::*;
use sgl::shader::*;
use sgl::shader_program::*;
fn draw_array(vao: &VertexArray,first: gl::types::GLint,size: gl::types::GLsizei) {
    vao.bind();
    gl::DrawArrays(gl::TRIANGLES, first,size);
}
fn draw_element_array(vao: &VertexArray,ibo: &IndexBuffer) {
    vao.bind();
    ibo.bind();
    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    // Run GLFW on the main thread
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    static vertices: [gl::types::GLfloat, ..6] = [
     -0.5, -0.5,
      0.5, -0.5,
     -0.5, 0.5
    ];
    static color: [gl::types::GLfloat, ..9] = [
      1.0, 0.0,0.0,
      0.0, 1.0,0.0,
      0.0, 0.0,1.0
    ];
    static indecies: [gl::types::GLuint, ..6] = [
     0,1,2,3,4,5
    ];
    do glfw::set_error_callback |_, description| {
        printfln!("GLFW Error: %s", description);
    }

    do glfw::start {
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint::context_version(4, 3);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);

        let window = glfw::Window::create(800, 600, "OpenGL", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load the OpenGL function pointers
        gl::load_with(glfw::get_proc_address);
        // Create Vertex Array Object
               
        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo = VertexBuffer::new(vertices,
                                    gl::STATIC_DRAW);
        let color_vbo = VertexBuffer::new(color,
                                          gl::STATIC_DRAW);

        let vertex_source = std::io::read_whole_file_str(&std::path::Path("triangle.vs")).unwrap();
        let fragment_source =  std::io::read_whole_file_str(&std::path::Path("triangle.fs")).unwrap();
        // Create and compile the vertex shader
        let vertex_shader = Shader::new_vs(~"triangle.vs",vertex_source).unwrap();
        // Create and compile the fragment shader
        let fragment_shader = Shader::new_fs(~"triangle.fs",fragment_source).unwrap();
        
        // Link the vertex and fragment shader into a shader program
        let shader_program = ShaderProgram::new([&vertex_shader,
                                                 &fragment_shader]).unwrap();
        shader_program.use_program();
        shader_program.set_uniform1f(~"value",1.0f32);
        //shader_program.bind_frag_location(1,~"123123outColor");
        let ibo = IndexBuffer::new(indecies,
                                   gl::STATIC_DRAW);
        let vao = VertexArray::new();
        vao.bind_attrib(shader_program.get_attrib_location("position"),
                        2,
                        &vbo);
        vao.bind_attrib(shader_program.get_attrib_location("color"),
                        3,
                        &color_vbo);  
        while !window.should_close() {
            // Poll events
            glfw::poll_events();
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            //vao.bind();
            draw_array(&vao,0,3);
            
            // Swap buffers
            window.swap_buffers();
        }
    }
}
