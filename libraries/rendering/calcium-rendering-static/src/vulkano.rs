use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use winit_window::{self, WinitWindow};

use calcium_rendering::{Error};
use calcium_rendering_vulkano::{VulkanoTypes, VulkanoWindowRenderer, VulkanoRenderer};

use {Initializer};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d_vulkano::{VulkanoWorld3DRenderer, VulkanoWorld3DTypes};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_vulkano::{VulkanoSimple2DTypes, VulkanoSimple2DRenderer};

pub struct VulkanoInitializer;

impl Initializer for VulkanoInitializer {
    type Types = VulkanoTypes;
    type Window = WinitWindow;

    #[cfg(feature = "world3d")]
    type World3DTypes = VulkanoWorld3DTypes;

    #[cfg(feature = "simple2d")]
    type Simple2DTypes = VulkanoSimple2DTypes;

    fn renderer(
        &self, log: Option<Logger>,
    ) -> Result<VulkanoRenderer, Error> {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));
        VulkanoRenderer::new(&log, winit_window::required_extensions())
    }

    fn window(
        &self,
        renderer: &VulkanoRenderer,
        title: &str, size: Vector2<u32>,
    ) -> Result<(WinitWindow, VulkanoWindowRenderer), Error> {
        let window = WinitWindow::new_vulkano(
            renderer.instance.clone(), title, [size.x, size.y].into()
        );
        let window_renderer = VulkanoWindowRenderer::new(
            renderer, window.surface.clone(), size
        );

        Ok((window, window_renderer))
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        renderer: &VulkanoRenderer,
    ) -> Result<VulkanoWorld3DRenderer, Error> {
        let world_renderer = VulkanoWorld3DRenderer::new(renderer);
        Ok(world_renderer)
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut VulkanoRenderer,
    ) -> Result<VulkanoSimple2DRenderer, Error> {
        VulkanoSimple2DRenderer::new(renderer)
    }
}
