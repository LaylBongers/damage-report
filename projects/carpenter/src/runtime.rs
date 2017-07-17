use cgmath::{Vector3, Quaternion, One};

use window::{Window, WindowSettings};
use slog::{Logger};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Types, Texture, TextureFormat};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, World3DTypes, Model, Material};
use calcium_rendering_static::{Runtime, Initializer};
use calcium_conrod::{ConrodRenderer};

use editor::{EditorWindow};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up the renderers and open up a window to render to
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, window, window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;

        // Set up the main editor window
        let mut editor_window = EditorWindow::new(
            &init, &mut renderer, &simple2d_renderer, window, window_renderer
        )?;

        // Run the actual game loop
        info!(self.log, "Finished loading, starting main loop");
        editor_window.run_loop(&init, &mut renderer, &mut simple2d_renderer)
    }
}
