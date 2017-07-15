use cgmath::{Vector2};
use input::{Input, Button, Key};
use window::{Window, WindowSettings};
use slog::{Logger};

use calcium_game::{LoopTimer};
use calcium_rendering::{Error, WindowRenderer, Texture, TextureFormat};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch, ShaderMode, DrawRectangle, Rectangle};
use calcium_rendering_static::{Runtime, Initializer};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up everything we need to render
        let window_settings = WindowSettings::new("Carpenter", [1280, 720]);
        let (mut renderer, mut window, mut window_renderer) =
            init.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = init.simple2d_renderer(&mut renderer)?;

        // Player data
        let player_texture = Texture::from_file(
            &mut renderer, "./assets/friendly.png", TextureFormat::Srgb,
        )?;
        let mut player: Vector2<f32> = Vector2::new(200.0, 200.0);
        let mut right_pressed = false;
        let player_size = Vector2::new(32.0, 32.0);

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();

            // Handle input
            while let Some(event) = window.poll_event() {
                // Pass the event to the window renderer
                window_renderer.handle_event(&event);

                match event {
                    Input::Press(Button::Keyboard(Key::D)) =>
                        right_pressed = true,
                    Input::Release(Button::Keyboard(Key::D)) =>
                        right_pressed = false,
                    _ => {}
                }
            }

            // Move the player
            if right_pressed {
                // Move the player
                player.x += delta * 64.0;
            }

            // Render a textured square for the player
            let mut batches = Vec::new();
            let mut batch = RenderBatch {
                mode: ShaderMode::Texture(player_texture.clone()),
                .. RenderBatch::default()
            };
            batch.rectangle(DrawRectangle {
                destination: Rectangle::new(player - player_size/2.0, player + player_size/2.0),
                .. DrawRectangle::default()
            });
            batches.push(batch);

            // Perform the rendering itself
            let mut frame = window_renderer.start_frame(&mut renderer);
            simple2d_renderer.render(&mut renderer, &mut frame, &batches);
            window_renderer.finish_frame(&mut renderer, frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
