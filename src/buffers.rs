use gl::types::*;

use crate::utils::gl_check_error;

// ==================================== Buffer ====================================================

/// An OpenGL buffer
#[derive(Debug)]
pub struct Buffer {
    /// ID of the buffer in OpenGL
    id: GLuint,
}

impl Buffer {
    pub fn create(data_ptr: *const u8, length: usize) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                length as isize,
                data_ptr as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        Buffer { id }
    }

    pub fn bind_as_array_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn bind_as_ebo(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
}

// ==================================== ElementBuffer =============================================

#[derive(Debug)]
pub struct ElementBuffer {
    pub num_elements: usize,
    pub buffer_offset: usize,
    pub element_type: GLenum,
}

// ==================================== VertexArray ===============================================

#[derive(Debug)]
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
        gl_check_error!();
        println!("Before bind is fine..");
        unsafe {
            gl::BindVertexArray(self.id);
        }
        gl_check_error!();
        println!("After bind is fine..");
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
        gl_check_error!();
    }
}
