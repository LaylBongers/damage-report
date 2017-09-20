use cgmath::{Vector2};
use gfx::{Device, Factory};

use calcium_rendering::{WindowRenderer};

use {GfxRenderer};

pub struct GfxWindowRenderer {
    size: Vector2<u32>,
}

impl GfxWindowRenderer {
    pub fn new(
        size: Vector2<u32>,
    ) -> Self {
        GfxWindowRenderer {
            size,
        }
    }

    pub fn report_resize(&mut self, size: Vector2<u32>) {
        self.size = size;
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    WindowRenderer<GfxRenderer<D, F>> for GfxWindowRenderer {
    fn start_frame(&mut self, renderer: &mut GfxRenderer<D, F>) -> GfxFrame {
        renderer.device.cleanup();

        GfxFrame {
            size: self.size
        }
    }

    fn finish_frame(&mut self, renderer: &mut GfxRenderer<D, F>, _frame: GfxFrame) {
        renderer.encoder.flush(&mut renderer.device);
    }

    fn size(&self) -> Vector2<u32> {
        self.size
    }
}

pub struct GfxFrame {
    size: Vector2<u32>,
}

impl GfxFrame {
    pub fn size(&self) -> Vector2<u32> {
        self.size
    }
}
