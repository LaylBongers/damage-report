use std::collections::hash_map::{DefaultHasher};
use std::sync::{Arc};
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

use cgmath::{Vector2, Vector3};
use slog::{Logger};

#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This is a potentially buggy hash function, but merging vertices this close together is
        //  acceptable, at least for now.
        (self.position * 10000.0).cast::<i64>().hash(state);
        (self.uv * 10000.0).cast::<i64>().hash(state);
        (self.normal * 10000.0).cast::<i64>().hash(state);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
        log: &Logger, flat_vertices: &Vec<Vertex>
    ) -> Arc<Mesh> {
        debug!(log, "Converting flat vertices to indexed";
            "vertices" => flat_vertices.len()
        );
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut lookup = HashMap::new();
        let mut i = 0;

        for vertex in flat_vertices {
            Self::find_or_add_vertex(
                vertex.clone(), &mut vertices, &mut indices, &mut lookup, &mut i
            );
        }

        Arc::new(Self::from_vertices_indices(vertices, indices))
    }

    fn find_or_add_vertex(
        vertex: Vertex,
        vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>, lookup: &mut HashMap<u64, u16>,
        i: &mut u16
    ) {
        // Check if we found a matchin vertex before
        let hash = calculate_hash(&vertex);
        if let Some(value) = lookup.get(&hash) {
            // We found a match, go with the existing index
            indices.push(*value);
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
