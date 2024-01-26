use std::{ffi::c_void, mem::size_of};

use crate::geometry::{Index, Vertex};

pub type VAO = u32;
pub type VBO = u32;
pub type EBO = u32;

pub struct Buffers {
    pub vbo: VBO,
    pub vao: VAO,
    pub ebo: EBO,
}

impl Buffers {
    pub fn new() -> Self {
        let mut vbo = 0;
        let mut vao = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut ebo);
        }
        Self {
            vbo, vao, ebo
        }
    }

    pub fn bind_vao(&self) {
        unsafe{gl::BindVertexArray(self.vao)};
    }
    pub fn bind_vbo(&self) {
        unsafe{gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo)};
    }
    pub fn bind_ebo(&self) {
        unsafe{gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo)};
    }
    pub fn data_vbo(vertices: &Vec<Vertex>, usage: u32) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<Vertex>()) as isize, vertices.as_ptr() as *const c_void, usage)
        }
    }
    pub fn data_ebo(indices: &Vec<Index>, usage: u32) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * size_of::<Index>()) as isize, indices.as_ptr() as *const c_void, usage)
        }
    }

    pub fn unbind_vao() {
        unsafe {gl::BindVertexArray(0);}
    }
}