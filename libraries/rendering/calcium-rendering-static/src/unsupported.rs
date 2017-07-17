#[cfg(feature = "world3d")]
mod world3d {
    use std::sync::{Arc};
    use calcium_rendering::{Types};
    use calcium_rendering_world3d::{World3DTypes, Vertex, Mesh, World3DRenderer, RenderWorld, Camera, World3DRenderTargetRaw, World3DRenderTarget};

    pub struct UnsupportedWorld3DTypes;

    impl<T: Types> World3DTypes<T> for UnsupportedWorld3DTypes {
        type Renderer = UnsupportedWorld3DRenderer;
        type RenderTargetRaw = UnsupportedWorld3DRenderTargetRaw;
        type Mesh = UnsupportedMesh;
    }

    pub struct UnsupportedWorld3DRenderer;

    impl<T: Types> World3DRenderer<T, UnsupportedWorld3DTypes> for UnsupportedWorld3DRenderer {
        fn render(
            &mut self, _world: &RenderWorld<T, UnsupportedWorld3DTypes>, _camera: &Camera,
            _world3d_rendertarget: &mut World3DRenderTarget<T, UnsupportedWorld3DTypes>,
            _renderer: &mut T::Renderer, _window_renderer: &mut T::WindowRenderer,
            _frame: &mut T::Frame
        ) {
            panic!("Unsupported!")
        }
    }

    pub struct UnsupportedWorld3DRenderTargetRaw;

    impl<T: Types> World3DRenderTargetRaw<T, UnsupportedWorld3DTypes>
        for UnsupportedWorld3DRenderTargetRaw {
        fn new(
            _should_clear: bool,
            _renderer: &T::Renderer, _window_renderer: &T::WindowRenderer,
            _world3d_renderer: &UnsupportedWorld3DRenderer,
        ) -> Self {
            UnsupportedWorld3DRenderTargetRaw {
            }
        }
    }


    pub struct UnsupportedMesh;

    impl<T: Types> Mesh<T> for UnsupportedMesh {
        fn new(
            _renderer: &T::Renderer, _vertices: Vec<Vertex>, _indices: Vec<u32>,
        ) -> Arc<Self> {
            panic!("Unsupported!")
        }
    }
}

#[cfg(feature = "world3d")]
pub use self::world3d::*;
