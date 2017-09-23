use calcium_rendering::{Error};

use {Backend, Context};

pub fn run_runtime<R: Runtime>(backend: Backend, runtime: R) -> Result<(), Error> {
    match backend {
        Backend::Vulkano =>
            runtime.run(::vulkano::VulkanoContext),
        Backend::GfxOpenGl =>
            runtime.run(::gfx_opengl::GfxOpenGlContext),
        Backend::GfxDirectX => unimplemented!(),
    }
}

pub trait Runtime {
    fn run<C: Context>(self, context: C) -> Result<(), Error>;
}
