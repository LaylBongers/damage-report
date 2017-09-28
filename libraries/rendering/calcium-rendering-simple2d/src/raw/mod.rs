use std::any::{Any};

use calcium_rendering::{Renderer, Frame};
use calcium_rendering::raw::{RendererRaw};

use render_data::{RenderData};
use {Simple2DRenderTarget};

pub trait Simple2DRendererRaw<R: RendererRaw>: Any + Sized {
    type RenderTargetRaw: Simple2DRenderTargetRaw<R, Self> + Any;

    fn render(
        &mut self,
        data: &RenderData<R>,
        frame: &mut Frame<R>,
        render_target: &mut Simple2DRenderTarget<R, Self>,
        renderer: &mut Renderer<R>,
    );
}

pub trait Simple2DRenderTargetRaw<R: RendererRaw, SR: Simple2DRendererRaw<R>> {
    fn new(
        clear: bool,
        renderer: &Renderer<R>,
        simple2d_renderer: &SR,
    ) -> Self;
}
