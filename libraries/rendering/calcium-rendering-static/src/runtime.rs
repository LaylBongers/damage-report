use cgmath::{Vector2};
use slog::{Logger};

use calcium_rendering::{Error};
use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoWindowRenderer, VulkanoRenderer};
use calcium_window_winit::{self, WinitWindow};

#[cfg(feature = "world3d")]
use calcium_rendering_world3d_vulkano::{VulkanoWorld3DRenderer, VulkanoWorldBackendTypes};

#[cfg(feature = "simple2d")]
use calcium_rendering_simple2d_vulkano::{VulkanoSimple2DBackendTypes, VulkanoSimple2DRenderer};

use {Backend, Initializer};

pub fn run_runtime<R: Runtime>(backend: Backend, runtime: R) -> Result<(), Error> {
    match backend {
        Backend::Vulkano => {
            runtime.run(VulkanoInitializer)
        },
        Backend::GfxOpenGl => unimplemented!(),
        Backend::GfxDirectX => unimplemented!(),
    }
}

pub trait Runtime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error>;
}

struct VulkanoInitializer;

impl Initializer for VulkanoInitializer {
    type BackendTypes = VulkanoBackendTypes;
    type Window = WinitWindow;

    #[cfg(feature = "world3d")]
    type World3DBackendTypes = VulkanoWorldBackendTypes;

    #[cfg(feature = "simple2d")]
    type Simple2DBackendTypes = VulkanoSimple2DBackendTypes;

    fn renderer(
        &self, log: &Logger,
    ) -> Result<VulkanoRenderer, Error> {
        VulkanoRenderer::new(log, calcium_window_winit::required_extensions())
    }

    fn window(
        &self, log: &Logger,
        renderer: &VulkanoRenderer,
        title: &str, size: Vector2<u32>,
    ) -> Result<(WinitWindow, VulkanoWindowRenderer), Error> {
        let window = WinitWindow::new_vulkano(renderer.instance.clone(), title, size);
        let window_renderer = VulkanoWindowRenderer::new(
            log, renderer, window.surface.clone(), size
        );

        Ok((window, window_renderer))
    }

    #[cfg(feature = "world3d")]
    fn world3d_renderer(
        &self, log: &Logger, renderer: &VulkanoRenderer,
    ) -> Result<VulkanoWorld3DRenderer, Error> {
        let world_renderer = VulkanoWorld3DRenderer::new(log, renderer);
        Ok(world_renderer)
    }

    #[cfg(feature = "simple2d")]
    fn simple2d_renderer(
        &self, log: &Logger, renderer: &VulkanoRenderer, window: &VulkanoWindowRenderer,
    ) -> Result<VulkanoSimple2DRenderer, Error> {
        Ok(VulkanoSimple2DRenderer::new(log, renderer, window))
    }
}
