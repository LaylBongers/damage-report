use cgmath::{Vector2};
use slog::{Logger};

use calcium_rendering::{Error};
use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoSystemContext, VulkanoWindowRenderer, VulkanoRenderer};
use calcium_window_winit::{self, WinitWindow};

use {Backend, Initializer};

pub fn run_runtime<R: StaticGameRuntime>(backend: Backend, runtime: R) -> Result<(), Error> {
    match backend {
        Backend::Vulkano => {
            runtime.run(VulkanoInitializer)
        },
        Backend::GfxOpenGl => unimplemented!(),
        Backend::GfxDirectX => unimplemented!(),
    }
}

pub trait StaticGameRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error>;
}

struct VulkanoInitializer;

impl Initializer for VulkanoInitializer {
    type BackendTypes = VulkanoBackendTypes;
    type Window = WinitWindow;

    fn system_context(
        &self, log: &Logger,
    ) -> Result<VulkanoSystemContext, Error> {
        VulkanoSystemContext::new(log, calcium_window_winit::required_extensions())
    }

    fn window(
        &self, system_context: &VulkanoSystemContext, title: &str, size: Vector2<u32>,
    ) -> (WinitWindow, VulkanoWindowRenderer) {
        let window = WinitWindow::new_vulkano(system_context.instance.clone(), title, size);
        let window_renderer = VulkanoWindowRenderer::new(window.surface.clone(), size);

        (window, window_renderer)
    }

    fn renderer(
        &self,
        log: &Logger, system_context: &VulkanoSystemContext,
        windows: &mut [&mut VulkanoWindowRenderer]
    ) -> Result<VulkanoRenderer, Error> {
        VulkanoRenderer::new(log, system_context, windows)
    }
}
