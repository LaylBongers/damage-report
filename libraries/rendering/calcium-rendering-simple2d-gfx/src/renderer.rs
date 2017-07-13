use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch};
use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxFrame};

pub struct GfxSimple2DRenderer;

impl GfxSimple2DRenderer {
    pub fn new() -> Self {
        GfxSimple2DRenderer
    }
}

impl Simple2DRenderer<GfxTypes> for GfxSimple2DRenderer {
    fn render(
        &mut self, _renderer: &mut GfxRenderer, _frame: &mut GfxFrame,
        _batches: &[RenderBatch<GfxTypes>]
    ) {
    }
}
