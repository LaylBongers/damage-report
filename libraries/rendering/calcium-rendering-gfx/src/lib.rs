extern crate cgmath;
extern crate gfx;
extern crate image;
#[macro_use]
extern crate slog;
extern crate calcium_rendering;

mod renderer;
mod texture;

pub use renderer::{GfxRenderer, GfxFrame};
pub use texture::{GfxTextureRaw, GenericView};

pub type ColorFormat = ::gfx::format::Rgba8;
pub type DepthFormat = ::gfx::format::DepthStencil;
