use gfx::{Device, Factory};

use calcium_rendering_gfx::{GfxRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTargetRaw};

use {GfxSimple2DRenderer};

pub struct GfxSimple2DRenderTargetRaw {
    clear: bool,
}

impl GfxSimple2DRenderTargetRaw {
    pub fn is_clear(&self) -> bool {
        self.clear
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderTargetRaw<GfxRenderer<D, F>, GfxSimple2DRenderer<D, F>> for GfxSimple2DRenderTargetRaw {
    fn new(
        clear: bool,
        _renderer: &GfxRenderer<D, F>,
        _simple2d_renderer: &GfxSimple2DRenderer<D, F>,
    ) -> Self {
        GfxSimple2DRenderTargetRaw {
            clear,
        }
    }
}
