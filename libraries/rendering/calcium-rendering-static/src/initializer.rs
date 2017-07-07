use cgmath::{Vector2};
use slog::{Logger};

use calcium_rendering::{Error, BackendTypes};
use calcium_window::{Window};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d::{WorldBackendTypes};

pub trait Initializer {
    type BackendTypes: BackendTypes;
    type Window: Window;

    #[cfg(feature = "world3d")]
    type WorldBackendTypes: WorldBackendTypes<Self::BackendTypes>;

    fn renderer(
        &self, log: &Logger,
    ) -> Result<<Self::BackendTypes as BackendTypes>::Renderer, Error>;

    fn window(
        &self, log: &Logger,
        renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
        title: &str, size: Vector2<u32>,
    ) -> (Self::Window, <Self::BackendTypes as BackendTypes>::WindowRenderer);

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self, log: &Logger, renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
    ) -> <Self::WorldBackendTypes as WorldBackendTypes<Self::BackendTypes>>::WorldRenderer;
}
