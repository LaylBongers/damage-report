use std::io::Cursor;

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4, Vector3, SquareMatrix};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{Surface, Frame, VertexBuffer, Program};
use image;

#[derive(Copy, Clone)]
struct Vertex {
    v_position: [f32; 3],
    v_tex_coords: [f32; 2],
}

implement_vertex!(Vertex, v_position, v_tex_coords);

pub struct Renderer {
    program: Program,
    texture: SrgbTexture2d,
}

impl Renderer {
    pub fn init(context: &Facade) -> Self {
        // Create the shader program to render with
        let vertex_shader_src = include_str!("./shader_vert.glsl");
        let fragment_shader_src = include_str!("./shader_frag.glsl");
        let program = Program::from_source(
            context,
            vertex_shader_src, fragment_shader_src,
            None
        ).unwrap();

        // Create the texture to render
        let image = image::load(
            Cursor::new(&include_bytes!("./texture.png")[..]),
            image::PNG
        ).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(
            image.into_raw(), image_dimensions
        );
        let texture = SrgbTexture2d::new(context, image).unwrap();

        Renderer {
            program,
            texture,
        }
    }

    pub fn render(&self, context: &Facade, frame: &mut Frame) {
        // Create the vertex buffer
        let vertex1 = Vertex { v_position: [-0.5, -0.5, 0.0], v_tex_coords: [0.0, 0.0] };
        let vertex2 = Vertex { v_position: [ 0.5, -0.5, 0.0], v_tex_coords: [1.0, 0.0] };
        let vertex3 = Vertex { v_position: [-0.5,  0.5, 0.0], v_tex_coords: [0.0, 1.0] };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = VertexBuffer::new(context, &shape).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Create the uniforms
        let perspective = PerspectiveFov {
            fovy: Rad::full_turn() * 0.27,
            aspect: 1280.0 / 720.0,
            near: 0.1,
            far: 500.0,
        };
        let projection = Matrix4::from(perspective);
        let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).invert().unwrap();
        let matrix_raw: [[f32; 4]; 4] = (projection * view).into();
        let uniforms = uniform! { u_matrix: matrix_raw, u_texture: &self.texture };

        // Perform the actual draw
        frame.draw(
            &vertex_buffer, &indices,
            &self.program,
            &uniforms,
            &Default::default(),
        ).unwrap();
    }
}
