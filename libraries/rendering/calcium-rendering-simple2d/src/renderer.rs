use std::any::{Any};
use calcium_rendering::{Renderer};
use {RenderBatch, Simple2DRenderTarget, Simple2DRenderTargetRaw};

/// A 2D renderer capable of rendering render batches.
pub trait Simple2DRenderer<R: Renderer>: Any + Sized {
    type RenderTargetRaw: Simple2DRenderTargetRaw<R, Self> + Any;

    /// Renders the given render batches to a frame.
    fn render(
        &mut self,
        batches: &[RenderBatch<R>],
        render_target: &mut Simple2DRenderTarget<R, Self>,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
        frame: &mut R::Frame,
    );
}
