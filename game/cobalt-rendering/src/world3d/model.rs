use std::path::{Path};
use std::rc::{Rc};
use std::io::{Read};
use std::fs::{File};

use glium::{VertexBuffer};
use wavefront_obj::obj::{self, Primitive, ObjSet, Object, VTNIndex};
use {Target};

#[derive(Copy, Clone)]
pub struct Vertex {
    v_position: [f32; 3],
    v_tex_coords: [f32; 2],
    v_normal: [f32; 3],
}

implement_vertex!(Vertex, v_position, v_tex_coords, v_normal);

/// A refcounted loaded model.
#[derive(Clone)]
pub struct Model {
    pub inner: Rc<GliumModel>
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
        let vertex_buffer = Self::create_vertex_buffer(target, &obj_set, scale);

        Model {
            inner: Rc::new(GliumModel {
                vertex_buffer
            })
        }
    }

    fn create_vertex_buffer(target: &Target, obj_set: &ObjSet, scale: f32) -> VertexBuffer<Vertex> {
        // A temporary vector to keep the vertices in
        let mut vertices = Vec::new();

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
                        vertices.push(Self::convert_vertex(v1, &object, scale));
                        vertices.push(Self::convert_vertex(v2, &object, scale));
                        vertices.push(Self::convert_vertex(v3, &object, scale));
                    }
                }
            }
        }

        // Finally, create the vertex buffer
        VertexBuffer::new(target.context(), &vertices).unwrap()
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

pub struct GliumModel {
    pub vertex_buffer: VertexBuffer<Vertex>,
}
