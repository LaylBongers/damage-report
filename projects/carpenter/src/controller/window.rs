use calcium_game::{LoopTimer};
use calcium_rendering::{Error};
use calcium_rendering_static::{Initializer};
use slog::{Logger};

use view::{WindowView};
use model::{MapEditor};
use input_manager::{InputManager};

pub struct WindowController {}

impl WindowController {
    pub fn new() -> Self {
        WindowController {}
    }

    pub fn run_loop<I: Initializer>(&mut self, log: &Logger, init: &I) -> Result<(), Error> {
        let mut input = InputManager::new();
        let mut timer = LoopTimer::start();

        // Model
        let mut editor = MapEditor::new();

        // View
        let mut window_view = WindowView::new(log, init, &mut editor)?;

        while !window_view.should_close() {
            let delta = timer.tick();

            window_view.handle_events(init, &mut input);
            window_view.update(delta, &mut input, &mut editor);
            window_view.render()?;
        }

        Ok(())
    }
}
