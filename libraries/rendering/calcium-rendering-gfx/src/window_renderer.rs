use cgmath::{Vector2};
use input::{Input};
use gfx::{Resources, Factory};

use calcium_rendering::{WindowRenderer};

use {GfxTypes, GfxRenderer};

pub struct GfxWindowRenderer {
}

impl GfxWindowRenderer {
    pub fn new() -> Self {
        GfxWindowRenderer {
        }
    }
}

impl<R: Resources, F: Factory<R> + 'static> WindowRenderer<GfxTypes<R, F>> for GfxWindowRenderer {
    fn handle_event(&mut self, _input: &Input) {
    }

    fn start_frame(&mut self, _renderer: &GfxRenderer<R, F>) -> GfxFrame {
        GfxFrame
    }

    fn finish_frame(&mut self, _renderer: &GfxRenderer<R, F>, _frame: GfxFrame) {
    }

    fn size(&self) -> Vector2<u32> {
        Vector2::new(10, 10)
    }
}

pub struct GfxFrame;
