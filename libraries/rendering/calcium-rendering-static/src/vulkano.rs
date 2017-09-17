use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use window::{WindowSettings};
use input::{Input};
use winit_window::{self, WinitWindow};

use calcium_rendering::{Error};
use calcium_rendering_vulkano::{VulkanoRenderer, VulkanoWindowRenderer};

use {Initializer};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d_vulkano::{VulkanoWorld3DRenderer};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_vulkano::{VulkanoSimple2DRenderer};

pub struct VulkanoInitializer;

impl Initializer for VulkanoInitializer {
    type Renderer = VulkanoRenderer;
    type Window = WinitWindow;

    #[cfg(feature = "world3d")]
    type World3DRenderer = VulkanoWorld3DRenderer;

    #[cfg(feature = "simple2d")]
    type Simple2DRenderer = VulkanoSimple2DRenderer;

    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) -> Result<(VulkanoRenderer, WinitWindow, VulkanoWindowRenderer), Error> {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));

        let renderer = VulkanoRenderer::new(&log, winit_window::required_extensions())?;
        let (window, window_renderer) = self.window(&renderer, window_settings)?;

        Ok((renderer, window, window_renderer))
    }

    fn window(
        &self,
        renderer: &VulkanoRenderer,
        window_settings: &WindowSettings,
    ) -> Result<(WinitWindow, VulkanoWindowRenderer), Error> {
        let window = WinitWindow::new_vulkano(
            renderer.instance().clone(), window_settings,
        );
        let size = window_settings.get_size();
        let window_renderer = VulkanoWindowRenderer::new(
            renderer, window.surface.clone(), Vector2::new(size.width, size.height),
        );

        Ok((window, window_renderer))
    }

    fn handle_event(
        &self,
        event: &Input,
        _renderer: &mut VulkanoRenderer,
        _window: &mut WinitWindow,
        window_renderer: &mut VulkanoWindowRenderer,
    ) {
        match event {
            &Input::Resize(w, h) =>
                window_renderer.queue_resize(Vector2::new(w, h)),
            _ => {}
        }
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        renderer: &mut VulkanoRenderer,
    ) -> Result<VulkanoWorld3DRenderer, Error> {
        VulkanoWorld3DRenderer::new(renderer)
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut VulkanoRenderer,
    ) -> Result<VulkanoSimple2DRenderer, Error> {
        VulkanoSimple2DRenderer::new(renderer)
    }
}
