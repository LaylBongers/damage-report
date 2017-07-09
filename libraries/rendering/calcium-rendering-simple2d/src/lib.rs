extern crate cgmath;
extern crate calcium_rendering;

mod backend_types;
mod render_commands;
mod renderer;

pub use backend_types::{Simple2DBackendTypes};
pub use render_commands::{RenderBatch, Rectangle};
pub use renderer::{Simple2DRenderer};
