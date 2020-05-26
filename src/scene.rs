use thiserror::Error;

use gl::types::*;
use glm::Mat4x4;
use gltf::accessor::DataType;
use gltf::Semantic::*;

use crate::buffers::{Buffer, ElementBuffer, VertexArray};
use crate::shader::{Program, ShaderError};

// ==================================== Error =====================================================

#[derive(Debug, Error)]
pub enum SceneError {
    #[error("Cannot import glTF model: {0}")]
    GltfError(#[from] gltf::Error),

    #[error("Shader error when drawing scene: {0}")]
    ShaderError(#[from] ShaderError),
}

// ==================================== Scene =====================================================

pub struct Scene {
    nodes: Vec<Node>,
    meshes: Vec<Mesh>, // @optimisation: make a flat array of primitives instead
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
        let mut nodes: Vec<Node> = document.nodes().map(Node::from_gltf).collect();

        // Store final transforms in each node
        let parent_transform = glm::identity();
        let scene = document.default_scene().unwrap();
        for gltf_node in scene.nodes() {
            recursive_update_transforms(gltf_node, &mut nodes, &parent_transform);
        }

        // Create meshes
        let meshes: Vec<Mesh> = document
            .meshes()
            .map(|mesh| Mesh::from_gltf(mesh, &buffers))
            .collect();

        Ok(Scene { nodes, meshes })
    }

    /// Draw all nodes in the scene
    pub fn draw(&self, proj: &Mat4x4, view: &Mat4x4, shader: &Program) -> Result<(), SceneError> {
        shader.set_used();
        shader.set_mat4("proj", proj)?;
        shader.set_mat4("view", view)?;
        for node in self.nodes.iter().filter(|n| n.mesh_id.is_some()) {
            shader.set_mat4("model", &node.transform)?;
            let mesh = &self.meshes[node.mesh_id.unwrap()];
            for primitive in mesh.primitives.iter() {
                primitive.draw();
            }
        }
        Ok(())
    }
}

/// Update the node and its children with the transform and store the final transforms
fn recursive_update_transforms(gltf_node: gltf::Node, nodes: &mut [Node], transform: &Mat4x4) {
    let node = &mut nodes[gltf_node.index()];

    // Apply parent transform
    let final_transform = transform * node.transform;
    node.transform = final_transform;

    for child in gltf_node.children() {
        recursive_update_transforms(child, nodes, &final_transform);
    }
}

// ==================================== Node ======================================================

#[derive(Debug)]
struct Node {
    mesh_id: Option<usize>,
    children_ids: Vec<usize>,

    /// The final transform matrix (including parent transforms)
    transform: Mat4x4,
}

impl Node {
    fn from_gltf(node: gltf::Node) -> Self {
        Node {
            mesh_id: node.mesh().map(|m| m.index()),
            children_ids: node.children().map(|n| n.index()).collect(),
            transform: Mat4x4::from(node.transform().matrix()),
        }
    }
}

// ==================================== Mesh ======================================================

#[derive(Debug)]
struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    fn from_gltf(mesh: gltf::Mesh, buffers: &[Buffer]) -> Self {
        let primitives = mesh
            .primitives()
            .map(|primitive| Primitive::from_gltf(primitive, buffers))
            .collect();
        Mesh { primitives }
    }
}

// ==================================== Primitive =================================================

#[derive(Debug)]
struct Primitive {
    vao: VertexArray,
    ebo: ElementBuffer,
}

impl Primitive {
    fn from_gltf(primitive: gltf::Primitive, buffers: &[Buffer]) -> Self {
        let indices = primitive.indices().unwrap();
        let ebo = ElementBuffer {
            num_elements: indices.count(),
            element_type: indices.data_type().as_gl_enum(),
            buffer_offset: indices.offset() + indices.view().unwrap().offset(),
        };

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
            let buffer_view = accessor.view().unwrap();

            let num_components = accessor.dimensions().multiplicity();
            let data_type = accessor.data_type();
            let stride = buffer_view.stride().unwrap_or(0);
            let offset = buffer_view.offset() + accessor.offset();

            let buffer = &buffers[buffer_view.buffer().index()];
            buffer.bind_as_ebo();
            unsafe {
                gl::VertexAttribPointer(
                    location,
                    num_components as i32,
                    data_type.as_gl_enum(),
                    gl::FALSE,
                    stride as i32,
                    offset as *const GLvoid,
                );
                gl::EnableVertexAttribArray(location);
            }

            if location == 0 {
                assert_eq!(data_type, DataType::F32);
                assert_eq!(num_components, 3);
            }
            if location == 1 {
                assert_eq!(data_type, DataType::F32);
                assert_eq!(num_components, 3);
            }
            if location == 2 {
                assert_eq!(data_type, DataType::F32);
                assert_eq!(num_components, 4);
            }

            // println!("== vertex attrib ==");
            // println!("   location: {:?}", location);
            // println!("   num_components {:?}", num_components);
            // println!("   data_type: {:?}", data_type);
            // println!("   stride: {:?}", stride);
            // println!("   offset: {:?}", offset);
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
