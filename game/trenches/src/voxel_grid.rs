use cgmath::{Vector3, Vector2};

use calcium_rendering_world3d::{Vertex};

pub struct VoxelGrid {
    size: Vector3<usize>,
    voxels: Vec<bool>,
}

impl VoxelGrid {
    pub fn new(size: Vector3<usize>) -> Self {
        VoxelGrid {
            size,
            voxels: vec![false; size.x * size.y * size.z],
        }
    }

    /// Returns none if no triangles.
    pub fn triangulate(&self) -> Option<Vec<Vertex>> {
        let mut vertices = Vec::new();

        // TODO: Eliminate invisible faces

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                for z in 0..self.size.z {
                    if !self.voxels[self.index_at(Vector3::new(x, y, z))] {
                        continue;
                    }

                    let offset = Vector3::new(x, y, z).cast();
                    add_cube(&mut vertices, offset);
                }
            }
        }

        if vertices.len() != 0 { Some(vertices) } else { None }
    }

    pub fn size(&self) -> Vector3<usize> {
        self.size
    }

    pub fn set_at(&mut self, position: Vector3<usize>, value: bool) {
        let index = self.index_at(position);
        self.voxels[index] = value;
    }

    fn index_at(&self, position: Vector3<usize>) -> usize {
        position.z + (position.y * self.size.z) + (position.x * self.size.z * self.size.y)
    }
}

fn add_cube(vertices: &mut Vec<Vertex>, offset: Vector3<usize>) {
    // Sides
    add_square(
        vertices, offset,
        Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, -1.0)
    );
    add_square(
        vertices, offset + Vector3::new(1, 0, 0),
        Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 0.0)
    );
    add_square(
        vertices, offset + Vector3::new(1, 0, 1),
        Vector3::new(-1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0)
    );
    add_square(
        vertices, offset + Vector3::new(0, 0, 1),
        Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(-1.0, 0.0, 0.0)
    );

    // Top/Bottom
    add_square(
        vertices, offset + Vector3::new(0, 1, 0),
        Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0)
    );
    add_square(
        vertices, offset + Vector3::new(0, 0, 1),
        Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, -1.0, 0.0)
    );
}

fn add_square(
    vertices: &mut Vec<Vertex>, offset: Vector3<usize>,
    tangent: Vector3<f32>, bitangent: Vector3<f32>, normal: Vector3<f32>
) {
    let offset = offset.cast();

    vertices.push(Vertex {
        position: offset,
        uv: Vector2::new(0.0, 1.0),
        normal,
    });
    vertices.push(Vertex {
        position: offset + bitangent,
        uv: Vector2::new(0.0, 0.0),
        normal,
    });
    vertices.push(Vertex {
        position: offset + tangent,
        uv: Vector2::new(1.0, 1.0),
        normal,
    });

    vertices.push(Vertex {
        position: offset + tangent + bitangent,
        uv: Vector2::new(1.0, 0.0),
        normal,
    });
    vertices.push(Vertex {
        position: offset + tangent,
        uv: Vector2::new(1.0, 1.0),
        normal,
    });
    vertices.push(Vertex {
        position: offset + bitangent,
        uv: Vector2::new(0.0, 0.0),
        normal,
    });
}
