use std::path::{Path};
use std::io::{Read};
use std::fs::{File};
use std::sync::{Arc};

use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use wavefront_obj::obj::{self, Primitive, ObjSet, Object, VTNIndex};
use {Target};

#[derive(Copy, Clone, PartialEq)]
pub struct Vertex {
    pub v_position: [f32; 3],
    pub v_tex_coords: [f32; 2],
    pub v_normal: [f32; 3],
}

impl_vertex!(Vertex, v_position, v_tex_coords, v_normal);

/// A refcounted loaded model.
#[derive(Clone)]
pub struct Model {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u16]>>
}

impl Model {
    pub fn load<P: AsRef<Path>>(target: &Target, path: P, scale: f32) -> Self {
        // TODO: Change unwraps to proper error handling
        // TODO: Expose separate objects in the object set as individual objects
        // TODO: Add logging (slog) support
        // TODO: Support indices

        // Load in the wavefront obj data
        let mut obj_file = File::open(path).unwrap();
        let mut obj_file_data = String::new();
        obj_file.read_to_string(&mut obj_file_data).unwrap();
        let obj_set = obj::parse(obj_file_data).unwrap();

        // Create the vertex buffer from the object set
        let (vertices, indices) = Self::obj_set_to_vertices(&obj_set, scale);
        println!("Loaded Model, Vertices: {} Indices: {}", vertices.len(), indices.len());

        // Finally, create the buffers
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            &target.device(), &BufferUsage::all(), Some(target.graphics_queue().family()),
            vertices.into_iter()
        ).unwrap();
        let index_buffer = CpuAccessibleBuffer::from_iter(
            &target.device(), &BufferUsage::all(), Some(target.graphics_queue().family()),
            indices.into_iter()
        ).unwrap();

        Model {
            vertex_buffer,
            index_buffer,
        }
    }

    fn obj_set_to_vertices(obj_set: &ObjSet, scale: f32) -> (Vec<Vertex>, Vec<u16>) {
        // A temporary vector to keep the vertices in
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut i = 0;

        // Go over all objects in the file
        for object in &obj_set.objects {
            // Skip empty
            if object.vertices.len() == 0 { continue; }

            // We don't have the same format for vertices in-engine as OBJ does so we have to
            //  convert them

            // Go through all sets of geometry (faces with material) in this object
            for geometry in &object.geometry {
                // Go through all shapes (grouped primitives, usually triangles) in the geometry
                for shape in &geometry.shapes {
                    // Make sure we got a triangle, it's the only shape we want to process
                    if let Primitive::Triangle(v1, v2, v3) = shape.primitive {
                        // Add the triangle's vertices to the vertices vector
                        Self::find_or_add_vertex(
                            Self::convert_vertex(v1, &object, scale),
                            &mut vertices, &mut indices, &mut i
                        );
                        Self::find_or_add_vertex(
                            Self::convert_vertex(v2, &object, scale),
                            &mut vertices, &mut indices, &mut i
                        );
                        Self::find_or_add_vertex(
                            Self::convert_vertex(v3, &object, scale),
                            &mut vertices, &mut indices, &mut i
                        );
                    }
                }
            }
        }

        (vertices, indices)
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

    fn convert_vertex(obj_vertex: VTNIndex, object: &Object, scale: f32) -> Vertex {
        let pos = object.vertices[obj_vertex.0];
        let tex = object.tex_vertices[obj_vertex.1.unwrap()];
        let norm = object.normals[obj_vertex.2.unwrap()];

        Vertex {
            v_position: [pos.x as f32 * scale, pos.y as f32 * scale, pos.z as f32 * scale],
            v_tex_coords: [tex.u as f32, tex.v as f32],
            v_normal: [norm.x as f32, norm.y as f32, norm.z as f32],
        }
    }
}
