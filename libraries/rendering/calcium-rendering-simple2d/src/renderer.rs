use std::any::{Any};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::{Renderer, Frame};

use {Simple2DRenderTarget, Simple2DRenderTargetRaw};
use render_data::{RenderData};

/// A 2D renderer capable of rendering render batches.
pub trait Simple2DRenderer<R: RendererRaw>: Any + Sized {
    type RenderTargetRaw: Simple2DRenderTargetRaw<R, Self> + Any;

    /// Renders a set of render data.
    fn render(
        &mut self,
        data: &RenderData<R>,
        frame: &mut Frame<R>,
        render_target: &mut Simple2DRenderTarget<R, Self>,
        renderer: &mut Renderer<R>,
    );
}
