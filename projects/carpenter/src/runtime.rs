use cgmath::{Vector2};
use conrod::{self, Widget, Positionable, Sizeable, Labelable, UiBuilder};
use conrod::text::{FontCollection};
use conrod::widget::{self, Text, Canvas, Button};
use window::{Window};
use slog::{Logger};

use calcium_game::{LoopTimer, AverageDelta, delta_to_fps};
use calcium_rendering::{Error, WindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderer};
use calcium_rendering_static::{Runtime, Initializer};
use calcium_conrod::{ConrodRenderer};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        let size = Vector2::new(1280, 720);

        // Set up everything we need to render
        let mut renderer = init.renderer(&self.log)?;
        let (mut window, mut window_renderer) = init.window(
            &self.log, &renderer, "Carpenter", Vector2::new(size.x, size.y)
        )?;
        let mut simple2d_renderer = init.simple2d_renderer(&self.log, &mut renderer)?;

        // Set up conrod and UI data
        let mut conrod_renderer: ConrodRenderer<I::BackendTypes> =
            ConrodRenderer::new(&self.log, &mut renderer);
        let mut ui_batches = vec!();
        let mut ui = UiBuilder::new(size.cast().into()).theme(theme()).build();
        ui.fonts.insert(FontCollection::from_bytes(::ttf_noto_sans::REGULAR).into_font().unwrap());
        let ids = Ids::new(ui.widget_id_generator());
        let mut count = 0;
        let mut text = String::from("Data");

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        let mut average_delta = AverageDelta::new();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();
            average_delta.accumulate(delta);

            // Poll for window events
            while let Some(event) = window.poll_event() {
                // Pass the event itself over to conrod
                if let Some(event) = ::conrod::backend::piston::event::convert(
                    event.clone(), size.x as f64, size.y as f64
                ) {
                    ui.handle_event(event);
                }
            }

            // Update the UI
            {
                let ui = &mut ui.set_widgets();

                // The root canvas
                Canvas::new()
                    .set(ids.canvas, ui);

                // FPS label
                Text::new(&format!("FPS: {}", delta_to_fps(average_delta.get())))
                    .top_right_of(ids.canvas)
                    .w(100.0)
                    .font_size(12)
                    .set(ids.fps_label, ui);
                Text::new(&format!("MS: {}", average_delta.get()))
                    .left_from(ids.fps_label, 12.0)
                    .w(100.0)
                    .font_size(12)
                    .set(ids.ms_label, ui);

                // Counter button
                for _click in Button::new()
                    .middle_of(ids.canvas)
                    .w_h(240.0, 80.0)
                    .label(&count.to_string())
                    .set(ids.counter, ui)
                {
                    count += 1;
                }

                for event in widget::TextBox::new(&text)
                    .down_from(ids.counter, 12.0)
                    .font_size(18)
                    .w_h(240.0, 36.0)
                    .set(ids.text_field, ui)
                {
                    match event {
                        widget::text_box::Event::Enter => info!(self.log, "TextBox {}", text),
                        widget::text_box::Event::Update(string) => text = string,
                    }
                }
            }

            // Create render batches for the UI
            conrod_renderer.draw_if_changed(
                &self.log, &mut renderer, &window_renderer, &mut ui, &mut ui_batches
            );

            // Perform the rendering itself
            let mut frame = window_renderer.start_frame(&renderer);
            simple2d_renderer.render(&mut renderer, &mut frame, &ui_batches);
            window_renderer.finish_frame(&renderer, frame);
        }

        Ok(())
    }
}

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: ::std::time::Duration::from_millis(500),
    }
}

widget_ids! { struct Ids {
    canvas,
    fps_label, ms_label,
    counter, text_field,
} }
