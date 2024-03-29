use slog::{Logger};
use window::{WindowSettings, Window};
use input::{Input, Button, ButtonArgs, ButtonState, Key};
use cgmath::{Vector2};

use calcium_rendering::{Error};
use calcium_rendering_2d::render_data::{RenderData};
use calcium_rendering_2d::{Renderer2DTarget};
use calcium_rendering_context::{Context, Runtime};
use calcium_game::{LoopTimer};

use model::{TileStructure};
use view::{BackgroundView, TileStructureView};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<C: Context>(self, context: C) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up everything we need to render
        let window_settings = WindowSettings::new("Space Game", [1280, 720]);
        let (mut renderer, mut window) =
            context.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = context.simple2d_renderer(&mut renderer)?;
        let mut simple2d_render_target = Renderer2DTarget::new(
            true, &renderer, &simple2d_renderer
        );

        // Set up models
        let mut tile_structure = TileStructure::empty(Vector2::new(100, 100));
        tile_structure.randomize_floors();

        // Set up views
        let background_view = BackgroundView::new(&mut renderer)?;
        let tile_structure_view = TileStructureView::new(&mut renderer)?;

        let mut _right_pressed = false;

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Starting main game loop");
        while !window.should_close() {
            let _delta = timer.tick();

            // Handle input
            while let Some(event) = window.poll_event() {
                // Let the context handle anything needed
                context.handle_event(&event, &mut renderer, &mut window);

                match event {
                    Input::Button(ButtonArgs {state, button, scancode: _scancode}) => {
                        let press = state == ButtonState::Press;
                        match button {
                            Button::Keyboard(Key::D) =>
                                _right_pressed = press,
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            // Set up the rendering data we'll need
            let mut render_data = RenderData::new();

            background_view.render(&mut render_data, &mut renderer);
            tile_structure_view.render(&tile_structure, &mut render_data);

            // Finally do the 2D rendering itself
            let mut frame = renderer.start_frame();
            simple2d_renderer.render(
                &render_data, &mut frame, &mut simple2d_render_target, &mut renderer
            );
            renderer.finish_frame(frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
