use calcium_rendering::{Types};

use {RenderWorld, World3DTypes, Camera, World3DRenderTarget};

pub trait World3DRenderer<T: Types, WT: World3DTypes<T>> {
    fn render(
        &mut self,
        world: &RenderWorld<T, WT>, camera: &Camera,
        world3d_rendertarget: &mut World3DRenderTarget<T, WT>,
        renderer: &mut T::Renderer, window_renderer: &mut T::WindowRenderer, frame: &mut T::Frame
    );
}
