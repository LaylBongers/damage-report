use std::sync::{Arc};

use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use cobalt_rendering::{Target};

#[derive(Copy, Clone, PartialEq)]
pub struct Vertex {
    pub v_position: [f32; 3],
    pub v_tex_coords: [f32; 2],
    pub v_normal: [f32; 3],
}

impl_vertex!(Vertex, v_position, v_tex_coords, v_normal);

/// An uploaded mesh. Internally ref-counted, cheap to clone.
#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u16]>>
}

impl Mesh {
    /// Creates a mesh from vertcies. Will eliminate duplicate vertices using indices. Avoid using
    /// if you can directly provide vertices/indices without duplicate checking instead.
    pub fn from_flat_vertices(target: &Target, flat_vertices: &Vec<Vertex>) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut i = 0;

        for vertex in flat_vertices {
            Self::find_or_add_vertex(*vertex, &mut vertices, &mut indices, &mut i);
        }

        Self::from_vertices_indices(target, &vertices, &indices)
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
        target: &Target, vertices: &Vec<Vertex>, indices: &Vec<u16>
    ) -> Mesh {
        // Finally, create the buffers
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            vertices.iter().map(|v| *v)
        ).unwrap();
        let index_buffer = CpuAccessibleBuffer::from_iter(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            indices.iter().map(|v| *v)
        ).unwrap();

        Mesh {
            vertex_buffer,
            index_buffer,
        }
    }
}
