use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_stdlog::{StdLog};
use glutin_window::{GlutinWindow};
use input::{Input};
use window::{WindowSettings};
use gfx::{Encoder};
use gfx_window_glutin::{self};
use gfx_device_gl::{Device, Factory};

use calcium_rendering::raw::{RawAccess};
use calcium_rendering::{Renderer, Error, CalciumErrorMappable};
use calcium_rendering_gfx::{GfxRendererRaw, ColorFormat, DepthFormat};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_gfx::{GfxSimple2DRenderer};

#[cfg(feature = "world3d")]
use unsupported::{UnsupportedWorld3DRenderer};

use {Context};

pub struct GfxOpenGlContext;

impl Context for GfxOpenGlContext {
    type RendererRaw = GfxRendererRaw<Device, Factory>;
    type Window = GlutinWindow;

    #[cfg(feature = "simple2d")]
    type Simple2DRenderer = GfxSimple2DRenderer<Device, Factory>;

    #[cfg(feature = "world3d")]
    type World3DRenderer = UnsupportedWorld3DRenderer;

    fn renderer(
        &self, log: Option<Logger>, window_settings: &WindowSettings,
    ) -> Result<
        (Renderer<GfxRendererRaw<Device, Factory>>, GlutinWindow),
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

        let renderer_raw = GfxRendererRaw::new(&log, device, factory, encoder, main_color, size);

        Ok((Renderer::raw_new(renderer_raw, log.clone()), window))
    }

    fn handle_event(
        &self,
        event: &Input,
        renderer: &mut Renderer<GfxRendererRaw<Device, Factory>>,
        window: &mut GlutinWindow,
    ) {
        match event {
            &Input::Resize(w, h) => {
                let (new_color, _new_depth) =
                    gfx_window_glutin::new_views::<ColorFormat, DepthFormat>(&window.window);
                renderer.raw_mut().set_color_view(new_color);
                renderer.raw_mut().report_resize(Vector2::new(w, h));
            },
            _ => {},
        }
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self,
        _renderer: &mut Renderer<GfxRendererRaw<Device, Factory>>,
    ) -> Result<UnsupportedWorld3DRenderer, Error> {
        Err(Error::Unsupported("world3d is not supported on this backend".to_string()))
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self,
        renderer: &mut Renderer<GfxRendererRaw<Device, Factory>>,
    ) -> Result<GfxSimple2DRenderer<Device, Factory>, Error> {
        GfxSimple2DRenderer::new(renderer)
    }
}
