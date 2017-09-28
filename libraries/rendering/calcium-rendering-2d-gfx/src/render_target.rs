use gfx::{Device, Factory};

use calcium_rendering::{Renderer};
use calcium_rendering_gfx::{GfxRendererRaw};
use calcium_rendering_2d::{Renderer2D};
use calcium_rendering_2d::raw::{Renderer2DTargetRaw};

use {GfxRenderer2DRaw};

pub struct GfxRenderer2DTargetRaw {
    clear: bool,
}

impl GfxRenderer2DTargetRaw {
    pub fn is_clear(&self) -> bool {
        self.clear
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Renderer2DTargetRaw<GfxRendererRaw<D, F>, GfxRenderer2DRaw<D, F>>
    for GfxRenderer2DTargetRaw
{
    fn new(
        clear: bool,
        _renderer: &Renderer<GfxRendererRaw<D, F>>,
        _simple2d_renderer: &Renderer2D<GfxRendererRaw<D, F>, GfxRenderer2DRaw<D, F>>,
    ) -> Self {
        GfxRenderer2DTargetRaw {
            clear,
        }
    }
}
