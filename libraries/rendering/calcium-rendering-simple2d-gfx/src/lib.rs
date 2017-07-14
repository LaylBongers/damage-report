extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate calcium_rendering_gfx;

mod renderer;
mod types;

pub use renderer::{GfxSimple2DRenderer};
pub use types::{GfxSimple2DTypes};
