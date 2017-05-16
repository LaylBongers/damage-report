use std::path::{Path};
use std::rc::{Rc};
use std::io::{Read};
use std::fs::{File};

use glium::{VertexBuffer};
use wavefront_obj::obj::{self, Primitive};
use {Target};

#[derive(Copy, Clone)]
pub struct Vertex {
    v_position: [f32; 3],
    v_tex_coords: [f32; 2],
}

implement_vertex!(Vertex, v_position, v_tex_coords);

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

        // A temporary vector to keep the vertices in
        let mut vertices = Vec::new();

        // Go over all objects in the file
        for obj in &obj_set.objects {
            // Skip empty
            if obj.vertices.len() == 0 { continue; }

            // We don't have the same format for vertices in-engine as OBJ does so we have to
            //  convert them

            // Go through all sets of geometry (faces with material) in this object
            for geometry in &obj.geometry {
                // Go through all shapes (grouped primitives, usually triangles) in the geometry
                for shape in &geometry.shapes {
                    // Make sure we got a triangle, it's the only shape we want to process
                    if let Primitive::Triangle(v1, v2, v3) = shape.primitive {
                        let v1pos = obj.vertices[v1.0];
                        let v1tex = obj.tex_vertices[v1.1.unwrap()];
                        let v2pos = obj.vertices[v2.0];
                        let v2tex = obj.tex_vertices[v2.1.unwrap()];
                        let v3pos = obj.vertices[v3.0];
                        let v3tex = obj.tex_vertices[v3.1.unwrap()];

                        // Add the triangle's vertices to the vertices vector
                        vertices.push(Vertex {
                            v_position: [v1pos.x as f32 * scale, v1pos.y as f32 * scale, v1pos.z as f32 * scale],
                            v_tex_coords: [v1tex.u as f32, v1tex.v as f32],
                        });
                        vertices.push(Vertex {
                            v_position: [v2pos.x as f32 * scale, v2pos.y as f32 * scale, v2pos.z as f32 * scale],
                            v_tex_coords: [v2tex.u as f32, v2tex.v as f32],
                        });
                        vertices.push(Vertex {
                            v_position: [v3pos.x as f32 * scale, v3pos.y as f32 * scale, v3pos.z as f32 * scale],
                            v_tex_coords: [v3tex.u as f32, v3tex.v as f32],
                        });
                    }
                }
            }
        }

        // Finally, create the vertex buffer
        let vertex_buffer = VertexBuffer::new(target.context(), &vertices).unwrap();

        Model {
            inner: Rc::new(GliumModel {
                vertex_buffer
            })
        }
    }
}

pub struct GliumModel {
    pub vertex_buffer: VertexBuffer<Vertex>,
}
