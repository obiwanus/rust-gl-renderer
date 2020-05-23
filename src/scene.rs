use thiserror::Error;

use gl::types::*;
use glm::Mat4x4;
use gltf::Semantic::*;

use crate::shader::{Program, ShaderError};

// ==================================== Error =====================================================

#[derive(Debug, Error)]
pub enum SceneError {
    #[error("Cannot import glTF model: {0}")]
    GltfError(gltf::Error),

    #[error("Shader error when drawing scene: {0}")]
    ShaderError(ShaderError),
}

impl From<gltf::Error> for SceneError {
    fn from(error: gltf::Error) -> Self {
        SceneError::GltfError(error)
    }
}

impl From<ShaderError> for SceneError {
    fn from(error: ShaderError) -> Self {
        SceneError::ShaderError(error)
    }
}

// ==================================== Scene =====================================================

pub struct Scene {
    nodes: Vec<Node>,
}

impl Scene {
    pub fn from(path: &str) -> Result<Self, SceneError> {
        let (document, buffer_data, _images) = gltf::import(path)?;

        // Create OpenGL buffers
        let buffers: Vec<Buffer> = buffer_data
            .into_iter()
            .map(|data| Buffer::create(data.as_ptr(), data.len()))
            .collect();

        // Create nodes
        let nodes: Vec<Node> = document
            .nodes()
            .filter_map(|node| Node::from(node, &buffers))
            .collect();

        Ok(Scene { nodes })
    }

    /// Draw all nodes in the scene
    pub fn draw(&self, proj: &Mat4x4, view: &Mat4x4, shader: &Program) -> Result<(), SceneError> {
        shader.set_mat4("proj", proj)?;
        shader.set_mat4("view", view)?;
        for node in self.nodes.iter() {
            shader.set_mat4("model", &node.transform)?;
            for primitive in node.primitives.iter() {
                primitive.draw();
            }
        }
        Ok(())
    }
}

// ==================================== Node ======================================================

struct Node {
    /// The node transform matrix
    transform: Mat4x4,
    primitives: Vec<Primitive>,
}

impl Node {
    fn from(node: gltf::Node, buffers: &[Buffer]) -> Option<Self> {
        Some(Node {
            transform: Mat4x4::from(node.transform().matrix()),
            primitives: node
                .mesh()?
                .primitives()
                .map(|primitive| Primitive::from(primitive, buffers))
                .collect(),
        })
    }
}

// ==================================== Primitive =================================================

struct Primitive {
    vao: VertexArray,
    ebo: ElementBuffer,
}

impl Primitive {
    fn from(primitive: gltf::Primitive, buffers: &[Buffer]) -> Self {
        let indices = primitive.indices().unwrap();
        let ebo = ElementBuffer {
            num_elements: indices.count(),
            element_type: indices.data_type().as_gl_enum(),
            buffer_offset: indices.offset(),
        };

        // Create buffers and describe attributes
        let vao = VertexArray::new();
        vao.bind();
        println!("vao: {:?})", vao);
        for (attr, accessor) in primitive.attributes() {
            let location: u32 = match attr {
                Positions => 0,
                Normals => 1,
                Colors(_) => 2,
                _ => continue, // skip the rest of attributes
            };
            let buffer_view = accessor.view().unwrap();

            let stride = buffer_view.stride().unwrap_or(0);
            let element_size = accessor.data_type().size();
            let offset = buffer_view.offset() + accessor.offset();

            buffers[buffer_view.buffer().index()].bind_as_array_buffer();
            buffers[buffer_view.buffer().index()].bind_as_ebo();
            unsafe {
                gl::VertexAttribPointer(
                    location,
                    accessor.dimensions().multiplicity() as i32,
                    accessor.data_type().as_gl_enum(),
                    gl::FALSE,
                    (stride * element_size) as i32,
                    offset as *const GLvoid,
                );
                gl::EnableVertexAttribArray(location);
            }
            println!("location: {:?}", location);
            println!(
                "accessor.dimensions().multiplicity() as i32: {:?}",
                accessor.dimensions().multiplicity() as i32
            );
            println!(
                "accessor.data_type().as_gl_enum(): {:?}",
                accessor.data_type().as_gl_enum()
            );
            println!(
                "(stride * element_size) as i32: {:?}",
                (stride * element_size) as i32
            );
            println!("offset as *const GLvoid: {:?}", offset as *const GLvoid);
        }
        vao.unbind(); // done

        Primitive { vao, ebo }
    }

    fn draw(&self) {
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ebo.num_elements as i32,
                self.ebo.element_type,
                self.ebo.buffer_offset as *const GLvoid,
            );
            assert_eq!(gl::GetError(), gl::NO_ERROR);
        }
    }
}

// ==================================== Buffer ====================================================

/// An OpenGL buffer
#[derive(Debug)]
struct Buffer {
    /// ID of the buffer in OpenGL
    id: GLuint,

    /// Size in bytes
    length: usize,
}

impl Buffer {
    fn create(data_ptr: *const u8, length: usize) -> Self {
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
        Buffer { id, length }
    }

    fn bind_as_array_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn bind_as_ebo(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

// ==================================== ElementBuffer =============================================

#[derive(Debug)]
struct ElementBuffer {
    num_elements: usize,
    buffer_offset: usize,
    element_type: GLenum,
}

// ==================================== VertexArray ===============================================

#[derive(Debug)]
struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
