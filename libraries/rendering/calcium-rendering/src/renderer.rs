use slog::{Logger};

pub trait Renderer {
    // Gets the slog logger associated with this renderer.
    fn log(&self) -> &Logger;
}
