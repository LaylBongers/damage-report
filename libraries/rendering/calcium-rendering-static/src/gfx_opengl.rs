use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use glutin_window::{GlutinWindow};
use window::{WindowSettings};

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
        let window: GlutinWindow = WindowSettings::new(title, size)
            .build()
            .map_platform_err()?;

        let window_renderer = GfxWindowRenderer::new();

        // TODO: Implementation note
        // OpenGL doesn't allow us to flexible create the context separate from the window like
        // vulkan does, so we need to create the context at this point. This complicates a lot of
        // things but as long as we have good single-window performance it's fine.

        // One option is to add window creation to renderer() as well and simply not allow window()
        // on some backends where it's exceptionally expensive.

        Ok((window, window_renderer))
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        _renderer: &mut GfxRenderer,
    ) -> Result<GfxSimple2DRenderer, Error> {
        Ok(GfxSimple2DRenderer::new())
    }
}
