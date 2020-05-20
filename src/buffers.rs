#![allow(dead_code)]

use gl::types::*;

pub struct VertexBuffer {
    id: GLuint,
    num_vertices: usize,
}

impl VertexBuffer {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        VertexBuffer {
            id,
            num_vertices: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_static_data(&mut self, vertex_data: &[f32], stride: usize) {
        self.num_vertices = vertex_data.len() / stride;
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                vertex_data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn draw_triangles(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices as i32);
        }
    }
}

pub struct ElementBuffer {
    id: GLuint,
    num_elements: usize,
}

impl ElementBuffer {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        ElementBuffer {
            id,
            num_elements: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn set_static_data(&mut self, data: &[u32], stride: usize) {
        self.num_elements = data.len() / stride;
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<u32>()) as isize,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn num_elements(&self) -> usize {
        self.num_elements
    }

    pub fn draw_triangles(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_elements as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_attrib(&self, location: u32, count: i32, stride: usize, offset: usize) {
        unsafe {
            gl::VertexAttribPointer(
                location,
                count,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as i32,
                (offset * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(location);
        }
    }
}
