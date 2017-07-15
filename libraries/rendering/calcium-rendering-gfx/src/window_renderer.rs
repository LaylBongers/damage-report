use cgmath::{Vector2};
use gfx::{Device, Factory};

use calcium_rendering::{WindowRenderer};

use {GfxTypes, GfxRenderer};

pub struct GfxWindowRenderer {
    pub size: Vector2<u32>,
}

impl GfxWindowRenderer {
    pub fn new(
        size: Vector2<u32>,
    ) -> Self {
        GfxWindowRenderer {
            size,
        }
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    WindowRenderer<GfxTypes<D, F>> for GfxWindowRenderer {
    fn start_frame(&mut self, renderer: &mut GfxRenderer<D, F>) -> GfxFrame {
        renderer.device.cleanup();

        renderer.encoder.clear(&renderer.color_view, [0.0, 0.0, 0.0, 1.0]);

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
    pub size: Vector2<u32>,
}
