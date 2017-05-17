use std::io::{Cursor};
use std::sync::{Arc};

use cgmath::{Rad, PerspectiveFov, Angle, Matrix4};
use image;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineParams};
use vulkano::pipeline::blend::{Blend};
use vulkano::pipeline::depth_stencil::{DepthStencil};
use vulkano::pipeline::input_assembly::{InputAssembly};
use vulkano::pipeline::multisample::{Multisample};
use vulkano::pipeline::vertex::{SingleBufferDefinition};
use vulkano::pipeline::viewport::{ViewportsState, Viewport, Scissor};
use vulkano::framebuffer::{Framebuffer, Subpass};

use world3d::{Camera, World, Entity};
use {Target, Frame};

#[allow(dead_code)]
mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/world3d/shader_vert.glsl")} }
#[allow(dead_code)]
mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/world3d/shader_frag.glsl")} }

pub struct Renderer {
    //program: Program,
    //texture: SrgbTexture2d,
}

impl Renderer {
    pub fn init(target: &Target) -> Self {
        // Load in the shaders
        let vs = vs::Shader::load(target.device()).unwrap();
        let fs = fs::Shader::load(target.device()).unwrap();

        // Set up a pipeline TODO: Comment better
        /*let pipeline_params = GraphicsPipelineParams {
            vertex_input: SingleBufferDefinition::new(),
            vertex_shader: vs.main_entry_point(),
            input_assembly: InputAssembly::triangle_list(),
            tessellation: None,
            geometry_shader: None,
            viewport: ViewportsState::Fixed {
                data: vec![(
                    Viewport {
                        origin: [0.0, 0.0],
                        depth_range: 0.0 .. 1.0,
                        dimensions: [
                            target.images()[0].dimensions()[0] as f32,
                            target.images()[0].dimensions()[1] as f32
                        ],
                    },
                    Scissor::irrelevant()
                )],
            },
            raster: Default::default(),
            multisample: Multisample::disabled(),
            fragment_shader: fs.main_entry_point(),
            depth_stencil: DepthStencil::disabled(),
            blend: Blend::pass_through(),
            render_pass: Subpass::from(render_pass.clone(), 0).unwrap(),
        };
        let pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<::world3d::Vertex>, _, _>> =
            Arc::new(GraphicsPipeline::new(target.device(), pipeline_params).unwrap());*/

        // Create the texture to render
        /*let image = image::load(
            Cursor::new(&include_bytes!("./texture.png")[..]),
            image::PNG
        ).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(
            image.into_raw(), image_dimensions
        );
        let texture = SrgbTexture2d::new(context, image).unwrap();*/

        Renderer {
            //program,
            //texture,
        }
    }

    /*pub fn render(&self, frame: &mut Frame, camera: &Camera, world: &World) {
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
