use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use window::{WindowSettings};
use input::{Input};
use winit_window::{self, WinitWindow};
use vulkano::instance::{Instance};

use calcium_rendering::raw::{RawAccess};
use calcium_rendering::{Renderer, Error, CalciumErrorMappable};
use calcium_rendering_vulkano::{VulkanoRendererRaw};

use {Context};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d_vulkano::{VulkanoWorld3DRenderer};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_vulkano::{VulkanoSimple2DRenderer};

pub struct VulkanoContext;

impl Context for VulkanoContext {
    type RendererRaw = VulkanoRendererRaw;
    type Window = WinitWindow;

    #[cfg(feature = "world3d")]
    type World3DRenderer = VulkanoWorld3DRenderer;

    #[cfg(feature = "simple2d")]
    type Simple2DRenderer = VulkanoSimple2DRenderer;

    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) -> Result<(Renderer<VulkanoRendererRaw>, WinitWindow), Error> {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));

        // Start by setting up the vulkano instance, this is a silo of vulkan that all our vulkan
        //  types will belong to
        debug!(log, "Creating vulkan instance");
        let instance = {
            // Tell it we need at least the extensions the window needs
            Instance::new(None, &winit_window::required_extensions(), None)
                .map_platform_err()?
        };

        // Set up the window
        let window = WinitWindow::new_vulkano(
            instance.clone(), window_settings,
        );
        let size = window_settings.get_size();

        // Set up the renderer itself
        let renderer = VulkanoRendererRaw::new(
            &log, instance,
            window.surface.clone(), Vector2::new(size.width, size.height)
        )?;

        Ok((Renderer::raw_new(renderer, log.clone()), window))
    }

    fn handle_event(
        &self,
        event: &Input,
        renderer: &mut Renderer<VulkanoRendererRaw>,
        _window: &mut WinitWindow,
    ) {
        match event {
            &Input::Resize(w, h) =>
                renderer.raw_mut().queue_resize(Vector2::new(w, h)),
            _ => {}
        }
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        renderer: &mut Renderer<VulkanoRendererRaw>,
    ) -> Result<VulkanoWorld3DRenderer, Error> {
        VulkanoWorld3DRenderer::new(renderer)
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut Renderer<VulkanoRendererRaw>,
    ) -> Result<VulkanoSimple2DRenderer, Error> {
        VulkanoSimple2DRenderer::new(renderer)
    }
}
