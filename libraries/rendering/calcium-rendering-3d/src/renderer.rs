use std::any::{Any};

use calcium_rendering::{Viewport, Renderer};

use {RenderWorld, Camera, World3DRenderTarget, MeshRaw, World3DRenderTargetRaw};

pub trait World3DRenderer<R: RendererRaw>: Any + Sized {
    type RenderTargetRaw: World3DRenderTargetRaw<R, Self> + Any;
    type MeshRaw: MeshRaw<R> + Any + Send + Sync;

    fn render(
        &mut self,
        world: &RenderWorld<R, Self>, camera: &Camera,
        world3d_rendertarget: &mut World3DRenderTarget<R, Self>, viewport: &Viewport,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer, frame: &mut R::Frame
    );
}
