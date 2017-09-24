use slog::{Logger};
use cgmath::{Vector2};
use gfx::{Device, Factory, Encoder};
use gfx::handle::{RenderTargetView};

use calcium_rendering::{Renderer};

use {ColorFormat, GfxTextureRaw};

pub struct GfxRenderer<D: Device, F: Factory<D::Resources>> {
    log: Logger,

    device: D,
    factory: F,
    encoder: Encoder<D::Resources, D::CommandBuffer>,

    color_view: RenderTargetView<D::Resources, ColorFormat>,
    size: Vector2<u32>,
}

impl<D: Device, F: Factory<D::Resources>> GfxRenderer<D, F> {
    pub fn new(
        log: &Logger,
        device: D, factory: F, encoder: Encoder<D::Resources, D::CommandBuffer>,

        color_view: RenderTargetView<D::Resources, ColorFormat>,
        size: Vector2<u32>,
    ) -> Self {
        info!(log, "Creating gfx renderer");

        GfxRenderer {
            log: log.clone(),

            device,
            factory,
            encoder,

            color_view,
            size,
        }
    }

    pub fn device(&self) -> &D {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    pub fn factory_mut(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn encoder(&self) -> &Encoder<D::Resources, D::CommandBuffer> {
        &self.encoder
    }

    pub fn encoder_mut(&mut self) -> &mut Encoder<D::Resources, D::CommandBuffer> {
        &mut self.encoder
    }

    pub fn color_view(&self) -> &RenderTargetView<D::Resources, ColorFormat> {
        &self.color_view
    }

    pub fn set_color_view(&mut self, color_view: RenderTargetView<D::Resources, ColorFormat>) {
        self.color_view = color_view
    }

    pub fn report_resize(&mut self, size: Vector2<u32>) {
        self.size = size;
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> Renderer for GfxRenderer<D, F> {
    type Frame = GfxFrame;
    type TextureRaw = GfxTextureRaw<D>;

    fn log(&self) -> &Logger {
        &self.log
    }

    fn size(&self) -> Vector2<u32> {
        self.size
    }

    fn start_frame(&mut self) -> GfxFrame {
        self.device.cleanup();

        GfxFrame {
            size: self.size
        }
    }

    fn finish_frame(&mut self, _frame: GfxFrame) {
        self.encoder.flush(&mut self.device);
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
