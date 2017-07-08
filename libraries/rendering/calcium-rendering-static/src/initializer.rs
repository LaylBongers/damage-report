use cgmath::{Vector2};
use slog::{Logger};

use calcium_rendering::{Error, BackendTypes};
use calcium_window::{Window};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d::{World3DBackendTypes};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d::{Simple2DBackendTypes};

pub trait Initializer {
    type BackendTypes: BackendTypes;
    type Window: Window;

    #[cfg(feature = "world3d")]
    type World3DBackendTypes: World3DBackendTypes<Self::BackendTypes>;

    #[cfg(feature = "simple2d")]
    type Simple2DBackendTypes: Simple2DBackendTypes<Self::BackendTypes>;

    fn renderer(
        &self, log: &Logger,
    ) -> Result<<Self::BackendTypes as BackendTypes>::Renderer, Error>;

    fn window(
        &self, log: &Logger,
        renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
        title: &str, size: Vector2<u32>,
    ) -> Result<(Self::Window, <Self::BackendTypes as BackendTypes>::WindowRenderer), Error>;

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self, log: &Logger, renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
    ) -> Result<
        <Self::World3DBackendTypes as World3DBackendTypes<Self::BackendTypes>>::Renderer,
        Error
    >;

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self, log: &Logger,
        renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
        window: &<Self::BackendTypes as BackendTypes>::WindowRenderer,
    ) -> Result<
        <Self::Simple2DBackendTypes as Simple2DBackendTypes<Self::BackendTypes>>::Renderer,
        Error
    >;
}
