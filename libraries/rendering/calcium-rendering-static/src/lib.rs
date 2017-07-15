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
extern crate calcium_rendering;
extern crate calcium_rendering_gfx;
extern crate calcium_rendering_vulkano;

#[cfg(feature = "world3d")]
extern crate calcium_rendering_world3d;
#[cfg(feature = "world3d")]
extern crate calcium_rendering_world3d_vulkano;

#[cfg(feature = "simple2d")]
extern crate calcium_rendering_simple2d;
#[cfg(feature = "simple2d")]
extern crate calcium_rendering_simple2d_gfx;
#[cfg(feature = "simple2d")]
extern crate calcium_rendering_simple2d_vulkano;

mod initializer;
mod runtime;
mod unsupported;

mod gfx_opengl;
mod vulkano;

pub use initializer::{Initializer};
pub use runtime::{run_runtime, Runtime};

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
