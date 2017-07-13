use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use glutin_window::{GlutinWindow};
use window::{WindowSettings};

use calcium_rendering::{Error, CalciumErrorMappable};
use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxWindowRenderer};

use {Initializer};

pub struct GfxOpenGlInitializer;

impl Initializer for GfxOpenGlInitializer {
    type Types = GfxTypes;
    type Window = GlutinWindow;

    #[cfg(feature = "world3d")]
    type World3DTypes = VulkanoWorld3DTypes;

    #[cfg(feature = "simple2d")]
    type Simple2DTypes = VulkanoSimple2DTypes;

    fn renderer(
        &self, log: Option<Logger>,
    ) -> Result<GfxRenderer, Error> {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));
        Ok(GfxRenderer::new(&log))
    }

    fn window(
        &self,
        _renderer: &GfxRenderer,
        title: &str, size: Vector2<u32>,
    ) -> Result<(GlutinWindow, GfxWindowRenderer), Error> {
        let size: [u32; 2] = size.into();
        let window_settings = WindowSettings::new(title, size);
        let window = GlutinWindow::new(&window_settings)
            .map_platform_err()?;

        let window_renderer = GfxWindowRenderer::new();

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
