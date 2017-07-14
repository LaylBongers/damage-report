use gfx::{self, Resources, Factory, VertexBuffer, ConstantBuffer, RenderTarget};
use gfx::traits::{FactoryExt};

use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxFrame, ColorFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch};

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "v_pos",
        color: [f32; 3] = "v_color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_transform",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        transform: ConstantBuffer<Transform> = "Transform",
        out: RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct GfxSimple2DRenderer;

impl GfxSimple2DRenderer {
    pub fn new<R: Resources, F: Factory<R> + 'static>(renderer: &mut GfxRenderer<R, F>) -> Self {
        let pso = renderer.factory.create_pipeline_simple(
            include_bytes!("../shaders/simple2d_150_vert.glsl"),
            include_bytes!("../shaders/simple2d_150_frag.glsl"),
            pipe::new()
        ).unwrap();

        GfxSimple2DRenderer
    }
}

impl<R: Resources, F: Factory<R> + 'static> Simple2DRenderer<GfxTypes<R, F>> for GfxSimple2DRenderer {
    fn render(
        &mut self, _renderer: &mut GfxRenderer<R, F>, _frame: &mut GfxFrame,
        _batches: &[RenderBatch<GfxTypes<R, F>>]
    ) {
    }
}
