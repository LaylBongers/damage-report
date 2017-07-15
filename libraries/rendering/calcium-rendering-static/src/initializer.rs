use slog::{Logger};
use window::{Window, WindowSettings};
use input::{Input};

use calcium_rendering::{Error, Types};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d::{World3DTypes};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d::{Simple2DTypes};

pub trait Initializer {
    type Types: Types;
    type Window: Window;

    #[cfg(feature = "world3d")]
    type World3DTypes: World3DTypes<Self::Types>;

    #[cfg(feature = "simple2d")]
    type Simple2DTypes: Simple2DTypes<Self::Types>;

    /// Creates a new renderer with an initial window.
    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) ->  Result<(
        <Self::Types as Types>::Renderer,
        Self::Window,
        <Self::Types as Types>::WindowRenderer,
    ), Error>;

    /// Creates additional windows.
    ///
    /// Only supported on the following backends:
    /// - Vulkano
    /// TODO: Add a system for requesting required features and reject backends that don't have it.
    fn window(
        &self,
        renderer: &<Self::Types as Types>::Renderer,
        window_settings: &WindowSettings,
    ) -> Result<(Self::Window, <Self::Types as Types>::WindowRenderer), Error>;

    /// Handles an event for a window, updating the renderers and window as needed. Using this the
    /// backend can resize its swapchain buffers and make other relevant changes.
    fn handle_event(
        &self,
        event: &Input,
        renderer: &mut <Self::Types as Types>::Renderer,
        window: &mut Self::Window,
        window_renderer: &mut <Self::Types as Types>::WindowRenderer,
    );

    /// Creates a world3d renderer.
    ///
    /// Only supported on the following backends:
    /// - Vulkano
    /// TODO: Add a system for requesting required features and reject backends that don't have it.
    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        renderer: &<Self::Types as Types>::Renderer,
    ) -> Result<
        <Self::World3DTypes as World3DTypes<Self::Types>>::Renderer,
        Error
    >;

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut <Self::Types as Types>::Renderer,
    ) -> Result<
        <Self::Simple2DTypes as Simple2DTypes<Self::Types>>::Renderer,
        Error
    >;
}
