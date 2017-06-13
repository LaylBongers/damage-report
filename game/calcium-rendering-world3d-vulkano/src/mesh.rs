use std::collections::{HashMap};
use std::sync::{Arc};

use cgmath::{Vector2, Vector3, InnerSpace};
use slog::{Logger};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use calcium_rendering_vulkano::{VulkanoRenderBackend};
use calcium_rendering_world3d::mesh::{Vertex, Mesh};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MeshId(usize);

pub struct VkVertex {
    pub v_position: [f32; 3],
    pub v_uv: [f32; 2],
    pub v_normal: [f32; 3],
    pub v_tangent: [f32; 3],
}

impl_vertex!(VkVertex, v_position, v_uv, v_normal, v_tangent);

pub struct VulkanoMeshBackend {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[VkVertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u16]>>
}

impl VulkanoMeshBackend {
    /// Creates a mesh from vertices and indices. Performs no duplicate checking.
    pub fn from_vertices_indices(
        log: &Logger, backend: &VulkanoRenderBackend,
        vertices: &Vec<Vertex>, indices: &Vec<u16>
    ) -> VulkanoMeshBackend {
        let mut hotfixed_uvs = false;
        let indices_len = indices.len();

        // Seed the tangent calculation data, we will accumulate data as we go over the triangles
        let mut tri_tangents = vec![TangentCalcEntry::new(); vertices.len()];

        // Go over all triangles and calculate tangents for them
        for tri in indices.chunks(3) {
            // Retrieve the relevant vertices
            let v0 = &vertices[tri[0] as usize];
            let v1 = &vertices[tri[1] as usize];
            let v2 = &vertices[tri[2] as usize];

            // First get the deltas for positions and UVs
            let delta_pos1 = v1.position - v0.position;
            let delta_pos2 = v2.position - v0.position;
            let mut delta_uv1 = v1.uv - v0.uv;
            let mut delta_uv2 = v2.uv - v0.uv;

            // Hotfix any bad UV data, most likely these don't have working normal maps anyways
            // If a model has this it probably just has a debug/single color texture applied
            let e = 0.0001;
            if (f32::abs(delta_uv1.x) < e && f32::abs(delta_uv1.y) < e) ||
               (f32::abs(delta_uv2.x) < e && f32::abs(delta_uv2.y) < e) ||
               (delta_uv1.x == 0.0 && delta_uv2.x == 0.0) ||
               (delta_uv1.y == 0.0 && delta_uv2.y == 0.0) {
                hotfixed_uvs = true;
                delta_uv1 = Vector2::new(0.0, 1.0);
                delta_uv2 = Vector2::new(1.0, 0.0);
            }

            // Now calculate the actual tangent from that
            let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);
            let tangent = Vector3::new(
                f * (delta_uv2.y * delta_pos1.x - delta_uv1.y * delta_pos2.x),
                f * (delta_uv2.y * delta_pos1.y - delta_uv1.y * delta_pos2.y),
                f * (delta_uv2.y * delta_pos1.z - delta_uv1.y * delta_pos2.z),
            ).normalize();

            // We panic on this just in case we didn't fix all bad UVs
            if tangent.x.is_nan() || tangent.y.is_nan() {
                error!(log, "NaN found");
                error!(log, "delta_uv1: {:?}", delta_uv1);
                error!(log, "delta_uv2: {:?}", delta_uv2);
                error!(log, "delta_pos1: {:?}", delta_pos1);
                error!(log, "delta_pos2: {:?}", delta_pos2);
                error!(log, "f: {:?}", f);
                error!(log, "tangent: {:?}", tangent);
                panic!();
            }

            // Store the tangent for these vertices
            tri_tangents[tri[0] as usize].add(tangent);
            tri_tangents[tri[1] as usize].add(tangent);
            tri_tangents[tri[2] as usize].add(tangent);
        }

        // Convert all vertices into final vertices taken by our shader
        // Here we also calculate the final tangent values, finishing the averaging process
        // Since CpuAccessibleBuffer::from_iter takes an iterator, we don't collect
        let vk_vertices = vertices.iter().enumerate().map(|(i, v)| VkVertex {
            v_position: v.position.into(),
            v_uv: v.uv.into(),
            v_normal: v.normal.into(),
            v_tangent: tri_tangents[i].average().into(),
        });

        // Finally, create the buffers
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            backend.device.clone(), BufferUsage::all(),
            Some(backend.graphics_queue.family()),
            vk_vertices
        ).unwrap();
        let index_buffer = CpuAccessibleBuffer::from_iter(
            backend.device.clone(), BufferUsage::all(),
            Some(backend.graphics_queue.family()),
            indices.iter().map(|v| *v)
        ).unwrap();

        // Log if we had to hotfix UVs
        if hotfixed_uvs {
            warn!(log, "Found triangles with bad UVs, tangents may be wrong");
        }

        debug!(log, "Created new mesh";
            "vertices" => vertices.len(), "indices" => indices_len
        );
        VulkanoMeshBackend {
            vertex_buffer,
            index_buffer,
        }
    }
}

/// Calculates average tangents for vertices.
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

pub struct BackendMeshes {
    meshes: HashMap<MeshId, VulkanoMeshBackend>,
}

impl BackendMeshes {
    pub fn new() -> BackendMeshes {
        BackendMeshes {
            meshes: HashMap::new(),
        }
    }

    // TODO: This is a near-exact copy of the backend texture loading system, make a common helper
    //  structure so the functionality can be rolled into that.
    pub fn request_mesh(
        &mut self, log: &Logger, target_backend: &VulkanoRenderBackend, texture: &Arc<Mesh>
    ) -> Option<&VulkanoMeshBackend> {
        // Look up the texture from the texture backend storage, or add it if it isn't there yet
        let mesh_backend = self.lookup_or_submit_mesh(log, target_backend, texture);

        // Right now it will be immediately ready
        // TODO: Offload mesh loading to a separate thread and check if it's ready
        Some(mesh_backend)
    }

    fn lookup_or_submit_mesh(
        &mut self, log: &Logger, target_backend: &VulkanoRenderBackend, texture: &Arc<Mesh>
    ) -> &VulkanoMeshBackend {
        let key = MeshId(arc_key(&texture));

        // If we don't have this texture yet, submit it first
        if !self.meshes.contains_key(&key) {
            self.submit_mesh(log, target_backend, texture);
        }

        self.meshes.get(&key).unwrap()
    }

    fn submit_mesh(&mut self, log: &Logger, target_backend: &VulkanoRenderBackend, mesh: &Arc<Mesh>) {
        // TODO: Offload loading to a separate thread

        // Start by loading in the actual mesh
        let mesh_backend = VulkanoMeshBackend::from_vertices_indices(
            log, target_backend, &mesh.vertices, &mesh.indices
        );

        // Store the mesh backend, maintaining its ID so we can look it back up
        self.store_mesh(&mesh, mesh_backend);
    }

    fn store_mesh(
        &mut self, texture: &Arc<Mesh>, texture_backend: VulkanoMeshBackend
    ) -> MeshId {
        let key = MeshId(arc_key(texture));

        // First make sure this texture doesn't already exist, this shouldn't ever happen, but it's
        // not that expensive to make sure
        if self.meshes.contains_key(&key) {
            panic!("Mesh backend already exists for mesh")
        }

        // Now that we're sure, we can submit the texture
        self.meshes.insert(key, texture_backend);

        key
    }
}

fn arc_key<T>(value: &Arc<T>) -> usize {
    value.as_ref() as *const T as usize
}
