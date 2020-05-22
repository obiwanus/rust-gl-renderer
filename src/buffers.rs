#![allow(dead_code)]

use gl::types::*;
use gltf::scene::Node;

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

    pub fn set_static_data(&mut self, data: &[u32]) {
        self.num_elements = data.len();
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
}

use gltf::Semantic::*;

pub struct Model {
    pub vao: VertexArray,
    pub transform: [[f32; 4]; 4],

    num_elements: usize,
    buffer_offset: usize,
}

impl Model {
    pub fn from(node: Node) -> Option<Self> {
        let mesh = node.mesh()?; // not all nodes have meshes

        // @notimplemented: more than one primitive
        let primitive = mesh.primitives().next().unwrap();
        let num_elements = primitive.indices().unwrap().count();
        let buffer_offset = primitive.indices().unwrap().offset();
        let transform = node.transform().matrix();

        // Create buffers and describe attributes
        let vao = VertexArray::new();
        vao.bind();
        for (attr, accessor) in primitive.attributes() {
            let location: u32 = match attr {
                Positions => 0,
                Normals => 1,
                Colors(_) => 2,
                _ => continue, // skip the rest of attributes
            };
            let stride = accessor.view().unwrap().stride().unwrap_or(0);
            let element_size = accessor.data_type().size();
            let offset = accessor.offset();
            unsafe {
                gl::VertexAttribPointer(
                    location,
                    accessor.dimensions().multiplicity() as i32,
                    accessor.data_type().as_gl_enum(),
                    gl::FALSE,
                    (stride * element_size) as i32,
                    (offset * element_size) as *const GLvoid,
                );
                gl::EnableVertexAttribArray(location);
            }
        }

        Some(Model {
            vao,
            transform,
            num_elements,
            buffer_offset,
        })
    }

    pub fn draw_triangles(&self, buffer_id: GLuint) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_elements as i32,
                gl::UNSIGNED_INT,
                self.buffer_offset as *const GLvoid,
            );
        }
    }
}
