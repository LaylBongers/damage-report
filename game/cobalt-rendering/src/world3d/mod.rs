use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::EmptyUniforms;
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
        let vertex1 = Vertex { v_position: [-0.5, -0.5, 0.0] };
        let vertex2 = Vertex { v_position: [ 0.5, -0.5, 0.0] };
        let vertex3 = Vertex { v_position: [ 0.0,  0.5, 0.0] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = VertexBuffer::new(context, &shape).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        frame.draw(
            &vertex_buffer, &indices,
            &self.program,
            &EmptyUniforms, &Default::default()
        ).unwrap();
    }
}
