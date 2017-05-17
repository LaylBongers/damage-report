use std::io::Cursor;

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use image;

use world3d::{Camera, World, Entity};
use {Frame};

pub struct Renderer {
    //program: Program,
    //texture: SrgbTexture2d,
}

impl Renderer {
    /*pub fn init(context: &Facade) -> Self {
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

    pub fn render(&self, frame: &mut Frame, camera: &Camera, world: &World) {
        // Create the uniforms
        let perspective = PerspectiveFov {
            fovy: Rad::full_turn() * 0.25,
            aspect: frame.size.x as f32 / frame.size.y as f32,
            near: 0.1,
            far: 500.0,
        };
        let projection = Matrix4::from(perspective);
        let view = camera.create_world_to_view_matrix();
        let projection_view = projection * view;

        // Set up the drawing parameters
        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        // Go over everything in the world
        for entity in &world.entities {
            self.render_entity(entity, frame, &params, &projection_view);
        }
    }

    fn render_entity(
        &self,
        entity: &Entity, frame: &mut Frame, params: &DrawParameters,
        projection_view: &Matrix4<f32>
    ) {
        // Create a matrix for this world entity
        let model = Matrix4::from_translation(entity.position);
        let matrix_raw: [[f32; 4]; 4] = (projection_view * model).into();

        // Perform the actual draw
        let uniforms = uniform! { u_matrix: matrix_raw, u_texture: &self.texture };
        frame.inner.draw(
            &entity.model.inner.vertex_buffer, &NoIndices(PrimitiveType::TrianglesList),
            &self.program, &uniforms,
            &params,
        ).unwrap();
    }*/
}
