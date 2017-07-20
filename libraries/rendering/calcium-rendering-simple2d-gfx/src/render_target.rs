use gfx::{Device, Factory};

use calcium_rendering_gfx::{GfxRenderer, GfxWindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderTargetRaw};

use {GfxSimple2DRenderer};

pub struct GfxSimple2DRenderTargetRaw {
    should_clear: bool,
}

impl GfxSimple2DRenderTargetRaw {
    pub fn should_clear(&self) -> bool {
        self.should_clear
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderTargetRaw<GfxRenderer<D, F>, GfxSimple2DRenderer<D, F>> for GfxSimple2DRenderTargetRaw {
    fn new(
        should_clear: bool,
        _renderer: &GfxRenderer<D, F>, _window_renderer: &GfxWindowRenderer,
        _simple2d_renderer: &GfxSimple2DRenderer<D, F>,
    ) -> Self {
        GfxSimple2DRenderTargetRaw {
            should_clear,
        }
    }
}
