extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate cgmath;
extern crate palette;
extern crate input;
extern crate rusttype;
extern crate image;
extern crate glyphlayout;

// TODO: Remove this and allow any font to be set
extern crate ttf_noto_sans;

mod element;
mod renderer;
pub mod style;
mod ui;

pub use element::{Element, ElementCursorState, ElementText};
pub use renderer::{UiRenderer};
pub use ui::{Ui, ElementId};
