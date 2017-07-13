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
        unimplemented!()
    }

    fn start_frame(&mut self, _renderer: &GfxRenderer) -> GfxFrame {
        unimplemented!()
    }

    fn finish_frame(&mut self, _renderer: &GfxRenderer, _frame: GfxFrame) {
        unimplemented!()
    }

    fn size(&self) -> Vector2<u32> {
        unimplemented!()
    }
}

pub struct GfxFrame;
