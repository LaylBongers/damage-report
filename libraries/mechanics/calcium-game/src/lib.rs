extern crate sys_info;
#[macro_use]
extern crate slog;

mod average_delta;
mod loop_timer;

use slog::{Logger};

pub use average_delta::{AverageDelta};
pub use loop_timer::{LoopTimer};

pub fn delta_to_fps(delta: f32) -> f32 {
    1.0 / delta
}

/// Logs system information, useful for debugging player logs and finding similarities between
/// players who are experiencing the same bug.
pub fn log_sys_info(log: &Logger) {
    info!(log, "# System Information #");
    info!(log, "CPU: {:?} logical cores, {:?} MHz",
        sys_info::cpu_num().unwrap_or(0),
        sys_info::cpu_speed().unwrap_or(0),
    );

    if let Ok(mem_info) = sys_info::mem_info() {
        info!(log, "Memory: {} free/{} total", mem_info.free, mem_info.total);
    } else {
        info!(log, "Memory: Couldn't Detect");
    }

    if let Ok(os_release) = sys_info::os_release() {
        info!(log, "OS Release: {}", os_release);
        if os_release == "6.2" {
            info!(log, "OS Release may be inaccurate, see Microsoft documentation");
        }
    } else {
        info!(log, "OS Release: Couldn't Detect");
    }

    if let Ok(os_type) = sys_info::os_type() {
        info!(log, "OS Type: {}", os_type);
    } else {
        info!(log, "OS Type: Couldn't Detect");
    }
    info!(log, "# System Information #");
}
