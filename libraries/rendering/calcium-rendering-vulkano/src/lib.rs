extern crate cgmath;
extern crate image;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate vulkano;
extern crate calcium_rendering;

mod renderer;
mod texture;
mod types;
mod window_renderer;
mod window_swapchain;

pub use renderer::{VulkanoRenderer};
pub use texture::{VulkanoTextureRaw};
pub use types::{VulkanoTypes};
pub use window_renderer::{VulkanoWindowRenderer, VulkanoFrame};
pub use window_swapchain::{WindowSwapchain};

// TODO: Go over all parts of this backend and make sure that values are exposed through accessors
//  instead of as direct fields. The assumption previously was that only the crate itself would use
//  these fields but this has since proven wrong.
