extern crate cgmath;
extern crate image;
#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate wavefront_obj;
pub extern crate winit;

pub mod world3d;
mod target;

pub use target::{Target, Event, Frame};
