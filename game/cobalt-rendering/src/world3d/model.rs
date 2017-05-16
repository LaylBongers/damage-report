use std::path::{Path};
use std::rc::{Rc};

use glium::{VertexBuffer};
use {Target};

#[derive(Copy, Clone)]
pub struct Vertex {
    v_position: [f32; 3],
    v_tex_coords: [f32; 2],
}

implement_vertex!(Vertex, v_position, v_tex_coords);

/// A refcounted loaded model.
#[derive(Clone)]
pub struct Model {
    pub inner: Rc<GliumModel>
}

impl Model {
    pub fn load<P: AsRef<Path>>(target: &Target, _path: P, _scale: f32) -> Self {
        let mut vertices = Vec::new();
        vertices.push(Vertex {v_position: [0.0, 0.0, 0.0], v_tex_coords: [0.0, 0.0]});
        vertices.push(Vertex {v_position: [1.0, 0.0, 0.0], v_tex_coords: [1.0, 0.0]});
        vertices.push(Vertex {v_position: [0.0, 1.0, 0.0], v_tex_coords: [0.0, 1.0]});
        let vertex_buffer = VertexBuffer::new(target.context(), &vertices).unwrap();

        Model {
            inner: Rc::new(GliumModel {
                vertex_buffer
            })
        }
    }
}

pub struct GliumModel {
    pub vertex_buffer: VertexBuffer<Vertex>,
}
