use slog::{Logger};
use cgmath::{Vector2};
use gfx::{Device, Factory, Encoder};
use gfx::handle::{RenderTargetView};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::{Frame};

use {ColorFormat, GfxTextureRaw};

pub struct GfxRendererRaw<D: Device, F: Factory<D::Resources>> {
    device: D,
    factory: F,
    encoder: Encoder<D::Resources, D::CommandBuffer>,

    color_view: RenderTargetView<D::Resources, ColorFormat>,
    size: Vector2<u32>,
}

impl<D: Device, F: Factory<D::Resources>> GfxRendererRaw<D, F> {
    pub fn new(
        log: &Logger,
        device: D, factory: F, encoder: Encoder<D::Resources, D::CommandBuffer>,

        color_view: RenderTargetView<D::Resources, ColorFormat>,
        size: Vector2<u32>,
    ) -> Self {
        info!(log, "Creating gfx renderer");

        GfxRendererRaw {
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

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> RendererRaw for GfxRendererRaw<D, F> {
    type FrameRaw = GfxFrameRaw;
    type TextureRaw = GfxTextureRaw<D>;

    fn size(&self) -> Vector2<u32> {
        self.size
    }

    fn start_frame(&mut self) -> Frame<Self> {
        self.device.cleanup();

        Frame::raw_new(GfxFrameRaw {
            size: self.size
        })
    }

    fn finish_frame(&mut self, _frame: Frame<Self>) {
        self.encoder.flush(&mut self.device);
    }
}

pub struct GfxFrameRaw {
    size: Vector2<u32>,
}

impl GfxFrameRaw {
    pub fn size(&self) -> Vector2<u32> {
        self.size
    }
}
