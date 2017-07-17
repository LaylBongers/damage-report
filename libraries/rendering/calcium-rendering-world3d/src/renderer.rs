use calcium_rendering::{Types};

use {RenderWorld, World3DTypes, Camera};

pub trait World3DRenderer<T: Types, WT: World3DTypes<T>> {
    fn render(
        &mut self,
        world: &RenderWorld<T, WT>, camera: &Camera,
        renderer: &mut T::Renderer, window_renderer: &mut T::WindowRenderer, frame: &mut T::Frame
    );
}