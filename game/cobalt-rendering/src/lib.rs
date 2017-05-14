extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;

pub mod world3d;
mod target;

pub use target::{Target, Event, Frame};
