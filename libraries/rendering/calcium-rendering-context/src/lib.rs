extern crate cgmath;
#[macro_use]
extern crate slog;
extern crate slog_stdlog;
extern crate input;
extern crate window;
extern crate winit_window;
extern crate glutin_window;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate vulkano;
extern crate calcium_rendering;
extern crate calcium_rendering_gfx;
extern crate calcium_rendering_vulkano;

#[cfg(feature = "3d")]
extern crate calcium_rendering_3d;
#[cfg(feature = "3d")]
extern crate calcium_rendering_3d_vulkano;

#[cfg(feature = "2d")]
extern crate calcium_rendering_2d;
#[cfg(feature = "2d")]
extern crate calcium_rendering_2d_gfx;
#[cfg(feature = "2d")]
extern crate calcium_rendering_2d_vulkano;

mod context;
mod runtime;
mod unsupported;

mod gfx_opengl_context;
mod vulkano_context;

pub use context::{Context};
pub use runtime::{run_runtime, Runtime};

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
