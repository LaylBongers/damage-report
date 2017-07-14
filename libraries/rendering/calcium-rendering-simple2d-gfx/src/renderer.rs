use cgmath::{self};
use gfx::{self, Device, Factory, VertexBuffer, ConstantBuffer};
use gfx::pso::{PipelineState};
use gfx::traits::{FactoryExt};

use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxFrame, ColorFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch};

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "v_position",
        uv: [f32; 2] = "v_uv",
        color: [f32; 4] = "v_color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_transform",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        transform: ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct GfxSimple2DRenderer<D: Device, F: Factory<D::Resources>> {
    pso: PipelineState<D::Resources, pipe::Meta>,
    _f: ::std::marker::PhantomData<F>,
}

impl<D: Device, F: Factory<D::Resources>> GfxSimple2DRenderer<D, F> {
    pub fn new(
        renderer: &mut GfxRenderer<D, F>
    ) -> Self {
        let pso = renderer.factory.create_pipeline_simple(
            include_bytes!("../shaders/simple2d_150_vert.glsl"),
            include_bytes!("../shaders/simple2d_150_frag.glsl"),
            pipe::new()
        ).unwrap();

        GfxSimple2DRenderer {
            pso,
            _f: Default::default(),
        }
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderer<GfxTypes<D, F>> for GfxSimple2DRenderer<D, F> {
    fn render(
        &mut self, renderer: &mut GfxRenderer<D, F>, frame: &mut GfxFrame,
        batches: &[RenderBatch<GfxTypes<D, F>>]
    ) {
        // Create a projection matrix that just matches coordinates to pixels
        let proj = cgmath::ortho(
            0.0, frame.size.x as f32,
            frame.size.y as f32, 0.0,
            1.0, -1.0
        );
        let transform = Transform {
            transform: proj.into()
        };
        let transform_buffer = renderer.factory.create_constant_buffer(1);
        renderer.encoder.update_buffer(&transform_buffer, &[transform], 0).unwrap();

        // Go over all batches
        for batch in batches {
            // Create a big mesh of all the rectangles we got told to draw this batch
            let mut vertices = Vec::new();
            for vertex in &batch.vertices {
                vertices.push(Vertex {
                    position: vertex.position.into(),
                    uv: vertex.uv.into(),
                    color: vertex.color.into(),
                });
            }

            // Create an actual VBO from it
            let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(
                &vertices, ()
            );

            // Gather together all the data we need to render
            let data = pipe::Data {
                vbuf: vertex_buffer,
                transform: transform_buffer.clone(),
                out: renderer.color_view.clone(),
            };

            // Finally, add the draw to the encoder
            renderer.encoder.draw(&slice, &self.pso, &data);
        }
    }
}
