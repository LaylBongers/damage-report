use calcium_rendering::{Renderer};
use {World3DRenderer};

pub struct World3DRenderTarget<R: Renderer, WR: World3DRenderer<R>> {
    pub raw: WR::RenderTargetRaw,
}

impl<R: Renderer, WR: World3DRenderer<R>> World3DRenderTarget<R, WR> {
    pub fn new(
        should_clear: bool,
        renderer: &R, window_renderer: &R::WindowRenderer,
        world3d_renderer: &WR,
    ) -> Self {
        let raw = World3DRenderTargetRaw::new(
            should_clear, renderer, window_renderer, world3d_renderer
        );

        World3DRenderTarget {
            raw,
        }
    }
}

pub trait World3DRenderTargetRaw<R: Renderer, WR: World3DRenderer<R>> {
    fn new(
        should_clear: bool,
        renderer: &R, window_renderer: &R::WindowRenderer,
        world3d_renderer: &WR,
    ) -> Self;
}
