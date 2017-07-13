use cgmath::{Vector2};
use input::{Input};

use calcium_rendering::{WindowRenderer};

use {GfxTypes, GfxRenderer};

pub struct GfxWindowRenderer;

impl GfxWindowRenderer {
    pub fn new() -> Self {
        GfxWindowRenderer
    }
}

impl WindowRenderer<GfxTypes> for GfxWindowRenderer {
    fn handle_event(&mut self, _input: &Input) {
    }

    fn start_frame(&mut self, _renderer: &GfxRenderer) -> GfxFrame {
        GfxFrame
    }

    fn finish_frame(&mut self, _renderer: &GfxRenderer, _frame: GfxFrame) {
    }

    fn size(&self) -> Vector2<u32> {
        Vector2::new(10, 10)
    }
}

pub struct GfxFrame;
