use std::sync::{Arc};

use cgmath::{Vector2, Vector3, InnerSpace};
use slog::{Logger};

use calcium_rendering::{Target};
use calcium_rendering_vulkano::{VulkanoTargetBackend};

#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

/// An uploaded mesh. Internally ref-counted, cheap to clone.
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    /// Creates a mesh from vertcies. Will eliminate duplicate vertices using indices. Avoid using
    /// if you can directly provide vertices/indices without duplicate checking instead.
    pub fn from_flat_vertices(
        log: &Logger, target: &Target<VulkanoTargetBackend>, flat_vertices: &Vec<Vertex>
    ) -> Arc<Mesh> {
        debug!(log, "Converting flat vertices to indexed";
            "vertices" => flat_vertices.len()
        );
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut i = 0;

        for vertex in flat_vertices {
            Self::find_or_add_vertex(vertex.clone(), &mut vertices, &mut indices, &mut i);
        }

        Arc::new(Self::from_vertices_indices(vertices, indices))
    }

    fn find_or_add_vertex(
        vertex: Vertex, vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>, i: &mut u16
    ) {
        // Check if the vector contains any matching vertex
        if let Some(value) = vertices.iter().enumerate().find(|v| *v.1 == vertex) {
            // We found a match, go with the existing one
            indices.push(value.0 as u16);
            return;
        }

        // We didn't find a match, create a new one
        vertices.push(vertex);
        indices.push(*i);
        *i += 1;
    }

    /// Creates a mesh from vertices and indices. Performs no duplicate checking.
    pub fn from_vertices_indices(
        vertices: Vec<Vertex>, indices: Vec<u16>
    ) -> Mesh {
        Mesh {
            vertices,
            indices,
        }
    }
}
