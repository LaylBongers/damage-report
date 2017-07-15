use slog::{Logger};
use gfx::{Device, Factory, Encoder};
use gfx::handle::RenderTargetView;

use calcium_rendering::{Renderer};

use {ColorFormat};

pub struct GfxRenderer<D: Device, F: Factory<D::Resources>> {
    pub log: Logger,
    pub device: D,
    pub factory: F,
    pub encoder: Encoder<D::Resources, D::CommandBuffer>,
    pub color_view: RenderTargetView<D::Resources, ColorFormat>,
}

impl<D: Device, F: Factory<D::Resources>> GfxRenderer<D, F> {
    pub fn new(
        log: &Logger,
        device: D, factory: F, encoder: Encoder<D::Resources, D::CommandBuffer>,
        color_view: RenderTargetView<D::Resources, ColorFormat>
    ) -> Self {
        info!(log, "Creating gfx renderer");

        GfxRenderer {
            log: log.clone(),
            device,
            factory,
            encoder,
            color_view,
        }
    }
}

impl<D: Device, F: Factory<D::Resources>> Renderer for GfxRenderer<D, F> {
    fn log(&self) -> &Logger {
        &self.log
    }
}
