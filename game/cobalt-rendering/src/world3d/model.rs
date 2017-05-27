use std::path::{Path};
use std::io::{Read};
use std::fs::{File};

use slog::{Logger};
use wavefront_obj::obj::{self, Primitive, ObjSet, Object, VTNIndex};

use world3d::{Mesh, Vertex};
use {Target};

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(log: &Logger, target: &Target, path: P, scale: f32) -> Self {
        // TODO: Change unwraps to proper error handling
        // TODO: Expose separate objects in the object set as individual objects
        // TODO: Add logging (slog) support
        // TODO: Support indices

        // Load in the wavefront obj data
        info!(log, "Loading model at \"{}\"", path.as_ref().display());
        let mut obj_file = File::open(path.as_ref()).unwrap();
        let mut obj_file_data = String::new();
        obj_file.read_to_string(&mut obj_file_data).unwrap();
        let obj_set = obj::parse(obj_file_data).unwrap();

        // Convert all the objects to meshes
        let meshes = Self::obj_set_to_vertex_vecs(target, &obj_set, scale);

        // Get the vertices and indices from the obj set
        /*let (vertices, indices) = Self::obj_set_to_vertices(&obj_set, scale);
        info!(
            log, "Loaded model at \"{}\", vertices: {} indices: {}",
            path.as_ref().display(),
            vertices.len(), indices.len()
        );*/

        Model {
            meshes
        }
    }

    fn obj_set_to_vertex_vecs(target: &Target, obj_set: &ObjSet, scale: f32) -> Vec<Mesh> {
        let mut meshes = Vec::new();

        // Go over all objects in the file
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
            meshes.push(Mesh::from_flat_vertices(target, &vertices));
        }

        meshes
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

    /*fn obj_set_to_vertices(obj_set: &ObjSet, scale: f32) -> (Vec<Vertex>, Vec<u16>) {
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
    }*/
}
