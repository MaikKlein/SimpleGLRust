extern mod gl;
extern mod glfw;
extern mod sgl;
use sgl::vertex_buffer::*;
use sgl::vertex_array::*;
use sgl::shader::*;
use sgl::shader_program::*;

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
    
    do glfw::set_error_callback |_, description| {
        printfln!("GLFW Error: %s", description);
    }

    do glfw::start {
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint::context_version(3, 2);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);

        let window = glfw::Window::create(800, 600, "OpenGL", glfw::Windowed).unwrap();
        window.make_context_current();

        // Load the OpenGL function pointers
        gl::load_with(glfw::get_proc_address);
        // Create Vertex Array Object
               
        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo = VertexBuffer::new();
        vbo.set_buffer_data(vertices,
                            gl::STATIC_DRAW);

        let vertex_source = std::io::read_whole_file_str(&std::path::Path("triangle.vs")).unwrap();
        let fragment_source =  std::io::read_whole_file_str(&std::path::Path("triangle.fs")).unwrap();
        // Create and compile the vertex shader
        let vertex_shader = Shader::new_vs(vertex_source).unwrap();
        // Create and compile the fragment shader
        let fragment_shader = Shader::new_fs(fragment_source).unwrap();
        
        // Link the vertex and fragment shader into a shader program
        let shader_program = ShaderProgram::new([&vertex_shader,
                                                 &fragment_shader]).unwrap();
        shader_program.use_program();
        //shader_program.bind_frag_location(1,~"123123outColor");
        let vao = VertexArray::new();
        vao.bind_attrib(shader_program.get_attrib_location("position"),
                        2,
                        &vbo); 
        while !window.should_close() {
            // Poll events
            glfw::poll_events();
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // Swap buffers
            window.swap_buffers();
        }
    }
}
