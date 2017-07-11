mod average_delta;
mod loop_timer;

pub use average_delta::{AverageDelta};
pub use loop_timer::{LoopTimer};

pub fn delta_to_fps(delta: f32) -> f32 {
    1.0 / delta
}
