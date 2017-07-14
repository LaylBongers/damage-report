use cgmath::{self};
use gfx::{self, Device, Factory, VertexBuffer, ConstantBuffer};
use gfx::pso::{PipelineState};
use gfx::traits::{FactoryExt};

use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxFrame, ColorFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch};

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "v_pos",
        color: [f32; 3] = "v_color",
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
        _batches: &[RenderBatch<GfxTypes<D, F>>]
    ) {
        const TRIANGLE: [Vertex; 3] = [
            Vertex { pos: [ 0.0, 0.0 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [ 0.0, 100.0 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [ 100.0, 0.0 ], color: [0.0, 0.0, 1.0] }
        ];

        // Create a projection matrix that just matches coordinates to pixels
        let proj = cgmath::ortho(
            0.0, frame.size.x as f32,
            frame.size.y as f32, 0.0,
            1.0, -1.0
        );
        let transform = Transform {
            transform: proj.into()
        };

        let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let transform_buffer = renderer.factory.create_constant_buffer(1);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            transform: transform_buffer,
            out: renderer.color_view.clone(),
        };
        renderer.encoder.update_buffer(&data.transform, &[transform], 0).unwrap();
        renderer.encoder.draw(&slice, &self.pso, &data);
    }
}
