use std::{ffi::c_void, mem::size_of, ptr::null};
use crate::{buffers::Buffers, shader::Shader};

pub struct Vertex {
    pub pos: [f32;3]
}

pub type Index = u32;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
    pub buffers: Buffers,
    pub shader: Shader,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Index>, shader: Shader) -> Self {
        let buffers = Buffers::new();
        buffers.bind_vao();
        buffers.bind_vbo();
        Buffers::data_vbo(&vertices, gl::STATIC_DRAW);
        buffers.bind_ebo();
        Buffers::data_ebo(&indices, gl::STATIC_DRAW);
        let mut attribute_offset = 0;
        for (index,vertex_attribute) in shader.vertex_attributes.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(index as u32, vertex_attribute.size as i32, vertex_attribute.attribute_type, if vertex_attribute.normalized {gl::TRUE} else {gl::FALSE}, size_of::<Vertex>() as i32, attribute_offset as *const c_void);
                attribute_offset += vertex_attribute.size * match vertex_attribute.attribute_type {
                    gl::UNSIGNED_BYTE | gl::BYTE => 1,
                    gl::UNSIGNED_SHORT | gl::SHORT => 2,
                    gl::FLOAT | gl::UNSIGNED_INT | gl::INT => 4,
                    gl::DOUBLE => 8,
                    _ => panic!("attribute type {} not recognized",vertex_attribute.attribute_type),
                };
                gl::EnableVertexAttribArray(index as u32);
            }
        }
        Self {
            vertices, indices, shader, buffers
        }
    }

    pub fn draw(&self) {
        self.shader.use_shader();
        self.buffers.bind_vao();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, null());
        }
        Buffers::unbind_vao();
    }
}