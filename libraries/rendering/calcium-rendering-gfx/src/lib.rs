extern crate cgmath;
extern crate gfx;
extern crate image;
#[macro_use]
extern crate slog;
extern crate calcium_rendering;

mod renderer;
mod texture;
mod window_renderer;

pub use renderer::{GfxRenderer};
pub use texture::{GfxTextureRaw, GenericView};
pub use window_renderer::{GfxWindowRenderer, GfxFrame};

pub type ColorFormat = ::gfx::format::Rgba8;
pub type DepthFormat = ::gfx::format::DepthStencil;

// TODO: Go over all parts of this backend and make sure that values are exposed through accessors
//  instead of as direct fields. The assumption previously was that only the crate itself would use
//  these fields but this has since proven wrong.
