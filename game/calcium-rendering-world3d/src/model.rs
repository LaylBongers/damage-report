use std::path::{Path};
use std::io::{Read};
use std::fs::{File};

use cgmath::{Vector2, Vector3};
use slog::{Logger};
use wavefront_obj::obj::{self, Primitive, ObjSet, Object, VTNIndex};

use calcium_rendering::{Target};
use calcium_rendering_vulkano::{VulkanoTargetBackend};
use {Mesh, Vertex};

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(
        log: &Logger, target: &Target<VulkanoTargetBackend>, path: P, scale: f32
    ) -> Self {
        // TODO: Change unwraps to proper error handling
        info!(log, "Loading model"; "path" => path.as_ref().display().to_string());

        // Load in the wavefront obj data
        debug!(log, "Loading obj file to string");
        let mut obj_file = File::open(path.as_ref()).unwrap();
        let mut obj_file_data = String::new();
        obj_file.read_to_string(&mut obj_file_data).unwrap();
        debug!(log, "Parsing obj file data");
        let obj_set = obj::parse(obj_file_data).unwrap();

        // Convert all the objects to meshes
        let meshes = Self::obj_set_to_meshes(log, target, &obj_set, scale);

        Model {
            meshes
        }
    }

    fn obj_set_to_meshes(
        log: &Logger, target: &Target<VulkanoTargetBackend>, obj_set: &ObjSet, scale: f32
    ) -> Vec<Mesh> {
        let mut meshes = Vec::new();

        // Go over all objects in the file
        debug!(log, "Converting {} objects to Meshes", obj_set.objects.len());
        for object in &obj_set.objects {
            // Skip empty objects
            if object.vertices.len() == 0 { continue; }

            // We don't have the same format for vertices in-engine as OBJ does so we have to
            //  convert them to flat vertices, then let Mesh's code index them. They're indexed
            //  differently than how we need them in the obj format.
            let mut vertices = Vec::new();

            // Go through all sets of geometry in this object, every set of geometry has one
            //  material assocated with it, but we don't support that, so we're assuming there's
            //  only one single material on the entire object.
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

            // Convert the vertices to a mesh
            meshes.push(Mesh::from_flat_vertices(log, target, &vertices));
        }

        meshes
    }

    fn convert_vertex(obj_vertex: VTNIndex, object: &Object, scale: f32) -> Vertex {
        let pos = object.vertices[obj_vertex.0];
        let tex = object.tex_vertices[obj_vertex.1.unwrap()];
        let norm = object.normals[obj_vertex.2.unwrap()];

        Vertex {
            position: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32) * scale,
            uv: Vector2::new(tex.u as f32, tex.v as f32),
            normal: Vector3::new(norm.x as f32, norm.y as f32, norm.z as f32),
        }
    }
}
