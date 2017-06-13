use std::collections::hash_map::{DefaultHasher};
use std::sync::{Arc};
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

use cgmath::{Vector2, Vector3};

#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    /// This is a potentially messy hash function, but merging vertices this close together is
    ///  acceptable.
    fn calculate_lossy_hash(&self) -> u64 {
        let mut state = DefaultHasher::new();

        let scale = 10_000.0;
        (self.position * scale).cast::<i64>().hash(&mut state);
        (self.uv * scale).cast::<i64>().hash(&mut state);
        (self.normal * scale).cast::<i64>().hash(&mut state);

        state.finish()
    }
}

/// An uploaded mesh. Internally ref-counted, cheap to clone.
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    /// Creates a mesh from vertices and indices. Performs no duplicate checking.
    pub fn new(
        vertices: Vec<Vertex>, indices: Vec<u16>
    ) -> Arc<Mesh> {
        Arc::new(Mesh {
            vertices,
            indices,
        })
    }
}

/// Converts a flat vertices vector to indexed vertices. Will eliminate duplicate vertices. Avoid
/// using if you can directly provide vertices/indices without duplicate checking instead.
pub fn flat_vertices_to_indexed(flat_vertices: &Vec<Vertex>) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut lookup = HashMap::new();
    let mut i = 0;

    for vertex in flat_vertices {
        find_or_add_vertex(
            vertex.clone(), &mut vertices, &mut indices, &mut lookup, &mut i
        );
    }

    (vertices, indices)
}

fn find_or_add_vertex(
    vertex: Vertex,
    vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>, lookup: &mut HashMap<u64, u16>,
    i: &mut u16
) {
    // Check if we found a matching vertex before. This is the reason we need a hash map of
    //  indices, a linear vector equality lookup would make this O(n^2). To generate the hash
    //  we make a few assumptions, it's quite a lossy hash but it should be good enough for
    //  most situations.
    let hash = vertex.calculate_lossy_hash();
    if let Some(value) = lookup.get(&hash) {
        // We found a match, go with the existing index
        indices.push(*value);
        return;
    }

    // We didn't find a match, create a new one
    vertices.push(vertex);
    indices.push(*i);
    lookup.insert(hash, *i);
    *i += 1;
}
