use calcium_rendering::{Types};
use {World3DTypes};

pub struct World3DRenderTarget<T: Types, WT: World3DTypes<T>> {
    pub raw: WT::RenderTargetRaw,
}

impl<T: Types, WT: World3DTypes<T>> World3DRenderTarget<T, WT> {
    pub fn new(
        should_clear: bool,
        renderer: &T::Renderer, window_renderer: &T::WindowRenderer,
        world3d_renderer: &WT::Renderer,
    ) -> Self {
        let raw = World3DRenderTargetRaw::new(
            should_clear, renderer, window_renderer, world3d_renderer
        );

        World3DRenderTarget {
            raw,
        }
    }
}

pub trait World3DRenderTargetRaw<T: Types, WT: World3DTypes<T>> {
    fn new(
        should_clear: bool,
        renderer: &T::Renderer, window_renderer: &T::WindowRenderer,
        world3d_renderer: &WT::Renderer,
    ) -> Self;
}
