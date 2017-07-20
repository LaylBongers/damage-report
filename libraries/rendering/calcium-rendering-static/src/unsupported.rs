#[cfg(feature = "world3d")]
mod world3d {
    use std::sync::{Arc};
    use calcium_rendering::{Renderer, Viewport};
    use calcium_rendering_world3d::{World3DRenderer, Vertex, Mesh, RenderWorld, Camera, World3DRenderTargetRaw, World3DRenderTarget};

    pub struct UnsupportedWorld3DRenderer;

    impl<R: Renderer> World3DRenderer<R> for UnsupportedWorld3DRenderer {
        type RenderTargetRaw = UnsupportedWorld3DRenderTargetRaw;
        type Mesh = UnsupportedMesh;

        fn render(
            &mut self, _world: &RenderWorld<R, Self>, _camera: &Camera,
            _world3d_rendertarget: &mut World3DRenderTarget<R, UnsupportedWorld3DRenderer>,
            _viewport: &Viewport,
            _renderer: &mut R, _window_renderer: &mut R::WindowRenderer,
            _frame: &mut R::Frame
        ) {
            panic!("Unsupported!")
        }
    }

    pub struct UnsupportedWorld3DRenderTargetRaw;

    impl<R: Renderer> World3DRenderTargetRaw<R, UnsupportedWorld3DRenderer>
        for UnsupportedWorld3DRenderTargetRaw {
        fn new(
            _should_clear: bool,
            _renderer: &R, _window_renderer: &R::WindowRenderer,
            _world3d_renderer: &UnsupportedWorld3DRenderer,
        ) -> Self {
            UnsupportedWorld3DRenderTargetRaw {
            }
        }
    }


    pub struct UnsupportedMesh;

    impl<R: Renderer> Mesh<R> for UnsupportedMesh {
        fn new(
            _renderer: &R, _vertices: Vec<Vertex>, _indices: Vec<u32>,
        ) -> Arc<Self> {
            panic!("Unsupported!")
        }
    }
}

#[cfg(feature = "world3d")]
pub use self::world3d::*;
