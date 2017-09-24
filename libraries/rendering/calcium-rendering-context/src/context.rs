use slog::{Logger};
use window::{Window, WindowSettings, AdvancedWindow};
use input::{Input};

use calcium_rendering::{Error, Renderer};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d::{Simple2DRenderer};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d::{World3DRenderer};

pub trait Context {
    type Renderer: Renderer;
    type Window: Window + AdvancedWindow;

    #[cfg(feature = "simple2d")]
    type Simple2DRenderer: Simple2DRenderer<Self::Renderer>;

    #[cfg(feature = "world3d")]
    type World3DRenderer: World3DRenderer<Self::Renderer>;

    /// Creates a new renderer with an initial window.
    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) ->  Result<(
        Self::Renderer,
        Self::Window,
    ), Error>;

    /// Handles an event for a window, updating the renderers and window as needed. Using this the
    /// backend can resize its swapchain buffers and make other relevant changes.
    fn handle_event(
        &self,
        event: &Input,
        renderer: &mut Self::Renderer,
        window: &mut Self::Window,
    );

    /// Creates a world3d renderer.
    ///
    /// Only supported on the following backends:
    /// - Vulkano
    /// TODO: Add a system for requesting required features and reject backends that don't have it.
    /// TODO: Remove WindowRenderer from this initialization, World3DRenderer should create a new
    ///  thing specific to a single window.
    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        renderer: &mut Self::Renderer,
    ) -> Result<
        Self::World3DRenderer,
        Error
    >;

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut Self::Renderer,
    ) -> Result<
        Self::Simple2DRenderer,
        Error
    >;
}