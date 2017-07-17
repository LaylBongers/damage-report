use slog::{Logger};
use window::{WindowSettings};

use calcium_rendering::{Error};
use calcium_rendering_static::{Runtime, Initializer};

use editor::{EditorWindow};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up the renderers and open up a window to render to
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, window, mut window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;
        let mut world3d_renderer = init.world3d_renderer(&mut renderer, &mut window_renderer)?;

        // Set up the main editor window
        let mut editor_window = EditorWindow::new(
            &mut renderer, &simple2d_renderer, &world3d_renderer, window, window_renderer
        )?;

        // Run the actual game loop
        info!(self.log, "Finished loading, starting main loop");
        editor_window.run_loop(&init, &mut renderer, &mut simple2d_renderer, &mut world3d_renderer)
    }
}
