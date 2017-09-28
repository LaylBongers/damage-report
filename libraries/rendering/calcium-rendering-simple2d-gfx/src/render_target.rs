use gfx::{Device, Factory};

use calcium_rendering::{Renderer};
use calcium_rendering_gfx::{GfxRendererRaw};
use calcium_rendering_simple2d::raw::{Simple2DRenderTargetRaw};

use {GfxSimple2DRendererRaw};

pub struct GfxSimple2DRenderTargetRaw {
    clear: bool,
}

impl GfxSimple2DRenderTargetRaw {
    pub fn is_clear(&self) -> bool {
        self.clear
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderTargetRaw<GfxRendererRaw<D, F>, GfxSimple2DRendererRaw<D, F>>
    for GfxSimple2DRenderTargetRaw
{
    fn new(
        clear: bool,
        _renderer: &Renderer<GfxRendererRaw<D, F>>,
        _simple2d_renderer: &GfxSimple2DRendererRaw<D, F>,
    ) -> Self {
        GfxSimple2DRenderTargetRaw {
            clear,
        }
    }
}
