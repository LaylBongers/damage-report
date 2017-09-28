use std::any::{Any};

use calcium_rendering::{Renderer, Frame};
use calcium_rendering::raw::{RendererRaw};

use render_data::{RenderData};
use {Renderer2DTarget, Renderer2D};

pub trait Renderer2DRaw<R: RendererRaw>: Any + Sized {
    type RenderTargetRaw: Renderer2DTargetRaw<R, Self> + Any;

    fn render(
        &mut self,
        data: &RenderData<R>,
        frame: &mut Frame<R>,
        render_target: &mut Renderer2DTarget<R, Self>,
        renderer: &mut Renderer<R>,
    );
}

pub trait Renderer2DTargetRaw<R: RendererRaw, SR: Renderer2DRaw<R>> {
    fn new(
        clear: bool,
        renderer: &Renderer<R>,
        simple2d_renderer: &Renderer2D<R, SR>,
    ) -> Self;
}
