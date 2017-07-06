use cgmath::{Vector2};
use slog::{Logger};

use calcium_rendering::{Error, BackendTypes};
use calcium_window::{Window};

pub trait Initializer {
    type BackendTypes: BackendTypes;
    type Window: Window;

    fn system_context(
        &self, log: &Logger,
    ) -> Result<<Self::BackendTypes as BackendTypes>::SystemContext, Error>;

    fn renderer(
        &self, log: &Logger,
        system_context: &<Self::BackendTypes as BackendTypes>::SystemContext,
    ) -> Result<<Self::BackendTypes as BackendTypes>::Renderer, Error>;

    fn window(
        &self, log: &Logger,
        system_context: &<Self::BackendTypes as BackendTypes>::SystemContext,
        renderer: &<Self::BackendTypes as BackendTypes>::Renderer,
        title: &str, size: Vector2<u32>,
    ) -> (Self::Window, <Self::BackendTypes as BackendTypes>::WindowRenderer);
}
