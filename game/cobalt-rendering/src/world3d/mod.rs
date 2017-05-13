use cgmath::{Rad, PerspectiveFov, Angle, Matrix4, Vector3, SquareMatrix};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Surface, Frame, VertexBuffer, Program};

#[derive(Copy, Clone)]
struct Vertex {
    v_position: [f32; 3],
}

implement_vertex!(Vertex, v_position);

pub struct Renderer {
    program: Program,
}

impl Renderer {
    pub fn init(context: &Facade) -> Self {
        let vertex_shader_src = include_str!("./shader_vert.glsl");
        let fragment_shader_src = include_str!("./shader_frag.glsl");
        let program = Program::from_source(
            context,
            vertex_shader_src, fragment_shader_src,
            None
        ).unwrap();

        Renderer {
            program,
        }
    }

    pub fn render(&self, context: &Facade, frame: &mut Frame) {
        // Create the vertex buffer
        let vertex1 = Vertex { v_position: [-0.5, -0.5, 0.0] };
        let vertex2 = Vertex { v_position: [ 0.5, -0.5, 0.0] };
        let vertex3 = Vertex { v_position: [ 0.0,  0.5, 0.0] };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = VertexBuffer::new(context, &shape).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Create the uniforms
        let perspective = PerspectiveFov {
            fovy: Rad::full_turn() * 0.25,
            aspect: 1280.0 / 720.0,
            near: 0.1,
            far: 100.0,
        };
        let projection = Matrix4::from(perspective);
        let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).invert().unwrap();
        let matrix_raw: [[f32; 4]; 4] = (projection * view).into();
        let uniforms = uniform! { u_matrix: matrix_raw };

        // Perform the actual draw
        frame.draw(
            &vertex_buffer, &indices,
            &self.program,
            &uniforms,
            &Default::default(),
        ).unwrap();
    }
}
