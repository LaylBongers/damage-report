use std::sync::{Arc};

use cgmath::{Vector2, Vector3, Point3, InnerSpace, MetricSpace};
use collision::{Sphere};
use slog::{Logger};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};

use calcium_rendering::{Renderer};
use calcium_rendering_vulkano::{VulkanoTypes, VulkanoRenderer};
use calcium_rendering_world3d::{Vertex, Mesh};

pub struct VulkanoMesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[VkVertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    pub culling_sphere: Sphere<f32>,
}

impl Mesh<VulkanoTypes> for VulkanoMesh {
    fn new(
        renderer: &VulkanoRenderer, vertices: Vec<Vertex>, indices: Vec<u32>,
    ) -> Arc<VulkanoMesh> {
        let indices_len = indices.len();

        // We need tangents for proper normal mapping
        let tri_tangents = calculate_tangents(&renderer.log(), &vertices, &indices);

        // We also need a culling sphere so we can avoid rendering unneeded meshes
        let culling_sphere = calculate_culling_sphere(&vertices);

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
            renderer.device().clone(), BufferUsage::all(),
            Some(renderer.graphics_queue().family()),
            vk_vertices
        ).unwrap();
        let index_buffer = CpuAccessibleBuffer::from_iter(
            renderer.device().clone(), BufferUsage::all(),
            Some(renderer.graphics_queue().family()),
            indices.iter().map(|v| *v)
        ).unwrap();

        debug!(renderer.log(), "Created new mesh";
            "vertices" => vertices.len(), "indices" => indices_len
        );
        Arc::new(VulkanoMesh {
            vertex_buffer,
            index_buffer,
            culling_sphere,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct MeshId(usize);

pub struct VkVertex {
    pub v_position: [f32; 3],
    pub v_uv: [f32; 2],
    pub v_normal: [f32; 3],
    pub v_tangent: [f32; 3],
}

impl_vertex!(VkVertex, v_position, v_uv, v_normal, v_tangent);

fn calculate_tangents(
    log: &Logger, vertices: &Vec<Vertex>, indices: &Vec<u32>
) -> Vec<TangentCalcEntry> {
    let mut hotfixed_uvs = false;

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

    // Log if we had to hotfix UVs
    if hotfixed_uvs {
        warn!(log, "Found triangles with bad UVs, tangents may be wrong");
    }

    tri_tangents
}

fn calculate_culling_sphere(vertices: &Vec<Vertex>) -> Sphere<f32> {
    // One point aint a mesh, man
    assert!(vertices.len() > 1);

    // This algorithm is probably suboptimal, but it finds an acceptable middle point in the mesh
    //  by using the two vertices furthest away from eachother, then picks the furthest point from
    //  there to get the radius the sphere should be. The reason this last step is needed is
    //  because a sphere around that point just covering those two point may not cover all points.

    // Start by getting the two points furthest away from eachother
    let p1 = find_furthest_point(vertices[0].position, vertices);
    let p2 = find_furthest_point(p1, vertices);

    // Get the middle point between those two, then get the furthest point from there
    let middle = p1 + ((p2 - p1) * 0.5);
    let furthest = find_furthest_point(middle, vertices);

    // Finally, create a sphere from the middle covering that point
    Sphere {
        center: Point3 { x: middle.x, y: middle.y, z: middle.z },
        radius: middle.distance(furthest),
    }
}

fn find_furthest_point(point: Vector3<f32>, vertices: &Vec<Vertex>) -> Vector3<f32> {
    let mut stored_point = point;
    let mut stored_distance_squared = 0.0;

    for vert in vertices.iter() {
        let distance_squared = point.distance2(vert.position);
        if distance_squared > stored_distance_squared {
            stored_point = vert.position;
            stored_distance_squared = distance_squared;
        }
    }

    return stored_point;
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
