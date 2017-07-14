use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use glutin_window::{GlutinWindow};
use window::{WindowSettings};
use gfx_window_glutin::{init_existing};

use calcium_rendering::{Error, CalciumErrorMappable};
use calcium_rendering_gfx::{GfxTypes, GfxRenderer, GfxWindowRenderer};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_gfx::{GfxSimple2DTypes, GfxSimple2DRenderer};

use {Initializer};

pub struct GfxOpenGlInitializer;

impl Initializer for GfxOpenGlInitializer {
    type Types = GfxTypes;
    type Window = GlutinWindow;

    #[cfg(feature = "simple2d")]
    type Simple2DTypes = GfxSimple2DTypes;

    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) -> Result<(GfxRenderer, GlutinWindow, GfxWindowRenderer), Error> {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));

        let window: GlutinWindow = window_settings
            .build()
            .map_platform_err()?;

        type ColorFormat = ::gfx::format::Rgba8;
        type DepthFormat = ::gfx::format::DepthStencil;
        let (_device, _factory, _main_color, _main_depth) =
            init_existing::<ColorFormat, DepthFormat>(&window.window);

        let renderer = GfxRenderer::new(&log);
        let window_renderer = GfxWindowRenderer::new();

        Ok((renderer, window, window_renderer))
    }

    fn window(
        &self,
        _renderer: &GfxRenderer,
        _window_settings: &WindowSettings,
    ) -> Result<(GlutinWindow, GfxWindowRenderer), Error> {
        Err(Error::Unsupported("window() is not supported on this backend".to_string()))
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        _renderer: &mut GfxRenderer,
    ) -> Result<GfxSimple2DRenderer, Error> {
        Ok(GfxSimple2DRenderer::new())
    }
}
