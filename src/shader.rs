use std::ptr::null;

pub struct Shader {
    pub program: u32,
    pub vertex_attributes: Vec<VertexAttribute>
}

pub struct VertexAttribute {
    pub attribute_type: u32,
    pub normalized: bool,
    pub size: usize,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str, vertex_attributes: Vec<VertexAttribute>) -> Self {
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_source.as_bytes().into_iter().map(|b| *b as i8).collect::<Vec<i8>>().as_ptr(), null());
            gl::CompileShader(vertex);
            gl::ShaderSource(fragment, 1, &fragment_source.as_bytes().into_iter().map(|b| *b as i8).collect::<Vec<i8>>().as_ptr(), null());
            gl::CompileShader(fragment);
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            Self { program, vertex_attributes }
        }
    }

    pub fn use_shader(&self) {
        unsafe{gl::UseProgram(self.program);}
    }
}