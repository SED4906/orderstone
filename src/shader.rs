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
            let program = gl::CreateProgram();
            let mut vertex_source = vertex_source.as_bytes().into_iter().map(|b| *b as i8).collect::<Vec<i8>>();
            vertex_source.push(0);
            let mut fragment_source = fragment_source.as_bytes().into_iter().map(|b| *b as i8).collect::<Vec<i8>>();
            fragment_source.push(0);
            gl::ShaderSource(vertex, 1, &vertex_source.as_ptr(), null());
            gl::CompileShader(vertex);
            // let mut compiled = 0;
            // let mut max_length = 0;
            // gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut compiled);
            // gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut max_length);
            // if compiled == 0 {
            //     let mut bytes = vec![0u8;max_length as usize];
            //     gl::GetShaderInfoLog(vertex, max_length, &mut max_length, bytes.as_mut_ptr() as *mut i8);
            //     println!("{}", String::from_utf8(bytes).unwrap());
            //     gl::DeleteShader(vertex);
            // }
            gl::ShaderSource(fragment, 1, &fragment_source.as_ptr(), null());
            gl::CompileShader(fragment);
            // let mut compiled = 0;
            // let mut max_length = 0;
            // gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut compiled);
            // gl::GetShaderiv(fragment, gl::INFO_LOG_LENGTH, &mut max_length);
            // if compiled == 0 {
            //     let mut bytes = vec![0u8;max_length as usize];
            //     gl::GetShaderInfoLog(fragment, max_length, &mut max_length, bytes.as_mut_ptr() as *mut i8);
            //     println!("{}", String::from_utf8(bytes).unwrap());
            //     gl::DeleteShader(fragment);
            // }
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
            // let mut linked = 0;
            // let mut max_length = 0;
            // gl::GetProgramiv(program, gl::LINK_STATUS, &mut linked);
            // gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut max_length);
            // if linked == 0 {
            //     let mut bytes = vec![0u8;max_length as usize];
            //     gl::GetProgramInfoLog(program, max_length, &mut max_length, bytes.as_mut_ptr() as *mut i8);
            //     println!("{}", String::from_utf8(bytes).unwrap());
            //     gl::DeleteProgram(program);
            // }
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            Self { program, vertex_attributes }
        }
    }

    pub fn use_shader(&self) {
        unsafe{gl::UseProgram(self.program);}
    }
}