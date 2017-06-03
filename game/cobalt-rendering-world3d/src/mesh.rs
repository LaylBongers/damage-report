use std::sync::{Arc};

use cgmath::{Vector2, Vector3, InnerSpace};
use slog::{Logger};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use cobalt_rendering::{Target};

#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

pub struct VkVertex {
    pub v_position: [f32; 3],
    pub v_uv: [f32; 2],
    pub v_normal: [f32; 3],
    pub v_tangent: [f32; 3],
}

impl_vertex!(VkVertex, v_position, v_uv, v_normal, v_tangent);

/// An uploaded mesh. Internally ref-counted, cheap to clone.
#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[VkVertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u16]>>
}

impl Mesh {
    /// Creates a mesh from vertcies. Will eliminate duplicate vertices using indices. Avoid using
    /// if you can directly provide vertices/indices without duplicate checking instead.
    pub fn from_flat_vertices(log: &Logger, target: &Target, flat_vertices: &Vec<Vertex>) -> Mesh {
        debug!(log, "Converting flat vertices to indexed";
            "vertices" => flat_vertices.len()
        );
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut i = 0;

        for vertex in flat_vertices {
            Self::find_or_add_vertex(vertex.clone(), &mut vertices, &mut indices, &mut i);
        }

        Self::from_vertices_indices(log, target, &vertices, &indices)
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
        log: &Logger, target: &Target, vertices: &Vec<Vertex>, indices: &Vec<u16>
    ) -> Mesh {
        let mut hotfixed_uvs = false;

        // Seed the tangent calculation data, we will accumulate data as we go over the triangles
        let mut tri_tangents = vec!(TangentCalcEntry::new(); vertices.len());

        // Go over all triangles and calculate tangents for them
        for tri in indices.chunks(3) {
            // Retrieve the relevant vertices
            let v0 = &vertices[tri[0] as usize];
            let v1 = &vertices[tri[1] as usize];
            let v2 = &vertices[tri[2] as usize];

            // First get the deltas for positions and UVs
            let edge1 = v1.position - v0.position;
            let edge2 = v2.position - v0.position;
            let mut delta_uv1 = v1.uv - v0.uv;
            let mut delta_uv2 = v2.uv - v0.uv;

            // Hotfix any bad UV data, most likely these don't have working normal maps anyways
            // If a model has this it probably just has a debug/single color texture applied
            let e = 0.0001;
            if (f32::abs(delta_uv1.x) < e && f32::abs(delta_uv1.y) < e) ||
               (f32::abs(delta_uv2.x) < e && f32::abs(delta_uv2.y) < e) {
                hotfixed_uvs = true;
                delta_uv1 = Vector2::new(0.0, 1.0);
                delta_uv2 = Vector2::new(1.0, 0.0);
            }

            // Now calculate the actual tangent from that
            let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
            let tangent = Vector3::new(
                f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x),
                f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y),
                f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z),
            ).normalize();

            // Store the tangent for these vertices
            tri_tangents[tri[0] as usize].add(tangent);
            tri_tangents[tri[1] as usize].add(tangent);
            tri_tangents[tri[2] as usize].add(tangent);
        }

        // Convert all vertices into final vertices taken by our shader
        // Here we also take out the final tangent values
        let vk_vertices: Vec<_> = vertices.iter().enumerate().map(|(i, v)| VkVertex {
            v_position: v.position.into(),
            v_uv: v.uv.into(),
            v_normal: v.normal.into(),
            v_tangent: tri_tangents[i].average().into(),
        }).collect();

        // Finally, create the buffers
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            vk_vertices.into_iter()
        ).unwrap();
        let index_buffer = CpuAccessibleBuffer::from_iter(
            target.device().clone(), BufferUsage::all(), Some(target.graphics_queue().family()),
            indices.iter().map(|v| *v)
        ).unwrap();

        // Log if we had to hotfix UVs
        if hotfixed_uvs {
            warn!(log, "Found triangles with two or more of the same UVs, tangents may be wrong");
        }

        debug!(log, "Created new mesh with";
            "vertices" => vertices.len(), "indices" => indices.len()
        );
        Mesh {
            vertex_buffer,
            index_buffer,
        }
    }
}

#[derive(Clone)]
struct TangentCalcEntry {
    value: Vector3<f32>,
    amount: i32,
}

impl TangentCalcEntry {
    fn new() -> Self {
        TangentCalcEntry {
            value: Vector3::new(0.0, 0.0, 0.0),
            amount: 0,
        }
    }

    fn add(&mut self, value: Vector3<f32>) {
        self.value += value;
        self.amount += 1;
    }

    fn average(&self) -> Vector3<f32> {
        (self.value / self.amount as f32).normalize()
    }
}
