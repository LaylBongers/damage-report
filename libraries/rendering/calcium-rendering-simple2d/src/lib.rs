extern crate cgmath;
extern crate calcium_rendering;
extern crate screenmath;

mod projection;
mod render_batch;
mod render_target;
mod renderer;

// Re-export screenmath types for convenience
pub use screenmath::{Rectangle};

pub use projection::{Projection};
pub use render_batch::{RenderBatch, DrawRectangle, ShaderMode, DrawVertex};
pub use render_target::{Simple2DRenderTarget, Simple2DRenderTargetRaw};
pub use renderer::{Simple2DRenderer, Simple2DRenderPassRaw, Simple2DRenderPass};
