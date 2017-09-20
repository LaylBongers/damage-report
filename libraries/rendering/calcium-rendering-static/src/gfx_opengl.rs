use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use glutin_window::{GlutinWindow};
use input::{Input};
use window::{WindowSettings};
use gfx::{Encoder};
use gfx_window_glutin::{self};
use gfx_device_gl::{Device, Factory};

use calcium_rendering::{Error, CalciumErrorMappable};
use calcium_rendering_gfx::{GfxRenderer, GfxWindowRenderer, ColorFormat, DepthFormat};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_gfx::{GfxSimple2DRenderer};

#[cfg(feature = "world3d")]
use unsupported::{UnsupportedWorld3DRenderer};

use {Initializer};

pub struct GfxOpenGlInitializer;

impl Initializer for GfxOpenGlInitializer {
    type Renderer = GfxRenderer<Device, Factory>;
    type Window = GlutinWindow;

    #[cfg(feature = "simple2d")]
    type Simple2DRenderer = GfxSimple2DRenderer<Device, Factory>;

    #[cfg(feature = "world3d")]
    type World3DRenderer = UnsupportedWorld3DRenderer;

    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) -> Result<
        (GfxRenderer<Device, Factory>, GlutinWindow, GfxWindowRenderer),
        Error
    > {
        let log = log.unwrap_or(Logger::root(StdLog.fuse(), o!()));

        let size = window_settings.get_size();
        let size = Vector2::new(size.width, size.height);
        let window: GlutinWindow = window_settings
            .build()
            .map_platform_err()?;

        let (device, mut factory, main_color, _main_depth) =
            gfx_window_glutin::init_existing::<ColorFormat, DepthFormat>(&window.window);
        let encoder: Encoder<_, _> = factory.create_command_buffer().into();

        let renderer = GfxRenderer::new(&log, device, factory, encoder, main_color);
        let window_renderer = GfxWindowRenderer::new(size);

        Ok((renderer, window, window_renderer))
    }

    fn window(
        &self,
        _renderer: &GfxRenderer<Device, Factory>,
        _window_settings: &WindowSettings,
    ) -> Result<(GlutinWindow, GfxWindowRenderer), Error> {
        Err(Error::Unsupported("window() is not supported on this backend".to_string()))
    }

    fn handle_event(
        &self,
        event: &Input,
        renderer: &mut GfxRenderer<Device, Factory>,
        window: &mut GlutinWindow,
        window_renderer: &mut GfxWindowRenderer,
    ) {
        match event {
            &Input::Resize(w, h) => {
                let (new_color, _new_depth) =
                    gfx_window_glutin::new_views::<ColorFormat, DepthFormat>(&window.window);
                renderer.color_view = new_color;
                window_renderer.report_resize(Vector2::new(w, h));
            },
            _ => {},
        }
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        _renderer: &mut GfxRenderer<Device, Factory>,
    ) -> Result<UnsupportedWorld3DRenderer, Error> {
        Err(Error::Unsupported("world3d is not supported on this backend".to_string()))
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut GfxRenderer<Device, Factory>,
    ) -> Result<GfxSimple2DRenderer<Device, Factory>, Error> {
        GfxSimple2DRenderer::new(renderer)
    }
}
