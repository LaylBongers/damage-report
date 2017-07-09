extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate calcium_rendering_static;
extern crate calcium_window;
extern crate cgmath;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use cgmath::{Vector2, Vector4};
use conrod::{widget, Widget, Positionable, Sizeable, Labelable};
use conrod::render::{PrimitiveWalker, PrimitiveKind};
use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium_rendering::{Error, WindowRenderer};
use calcium_rendering_simple2d::{RenderCommands, Simple2DRenderer, Rectangle};
use calcium_rendering_static::{Backend, Runtime, Initializer};
use calcium_window::{Window};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    info!(log, "Carpenter Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = run_game(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}


fn run_game(log: &Logger) -> Result<(), Error> {
    // TODO: Read in from configuration and UI
    let backend = Backend::Vulkano;

    // Run the game's runtime with the appropriate backends
    calcium_rendering_static::run_runtime(backend, StaticRuntime { log: log.clone() })
}

struct StaticRuntime {
    log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        let size = Vector2::new(1280, 720);
        let half_size: Vector2<i32> = size.cast() / 2;

        // Set up everything we need to render
        let renderer = init.renderer(&self.log)?;
        let (mut window, mut window_renderer) = init.window(
            &self.log, &renderer, "Carpenter", Vector2::new(size.x, size.y)
        )?;
        let mut simple2d_renderer = init.simple2d_renderer(&self.log, &renderer, &window_renderer)?;

        // Set up conrod
        let mut ui = conrod::UiBuilder::new(size.cast().into()).theme(theme()).build();
        let ids = Ids::new(ui.widget_id_generator());
        let mut count = 0;

        // Run the actual game loop
        info!(self.log, "Finished loading, starting main loop");
        while window.handle_events() {
            // Update the UI
            {
                let ui = &mut ui.set_widgets();
                widget::Canvas::new().pad(40.0).set(ids.canvas, ui);
                for _click in widget::Button::new()
                    .middle_of(ids.canvas)
                    .w_h(80.0, 80.0)
                    .label(&count.to_string())
                    .set(ids.counter, ui)
                {
                    count += 1;
                }
            }

            // Perform rendering
            let mut frame = window_renderer.start_frame();

            let mut cmds = RenderCommands::default();
            let mut prims = ui.draw();
            while let Some(prim) = prims.next_primitive() {
                match prim.kind {
                    PrimitiveKind::Rectangle { color } => {
                        let r = prim.rect;
                        let c = color.to_rgb();
                        cmds.rectangles.push(Rectangle {
                            start: Vector2::new(r.x.start, r.y.start).cast() + half_size,
                            size: Vector2::new(r.x.end - r.x.start, r.y.end - r.y.start).cast(),
                            color: Vector4::new(c.0, c.1, c.2, c.3),
                        });
                    },
                    PrimitiveKind::Polygon { color, points } => {
                        warn!(self.log, "conrod polygon unhandled");
                    },
                    PrimitiveKind::Lines { color, cap, thickness, points } => {
                        warn!(self.log, "conrod lines unhandled");
                    }
                    PrimitiveKind::Image { image_id, color, source_rect } => {
                        warn!(self.log, "conrod image unhandled");
                    }
                    PrimitiveKind::Text { color, text, font_id } => {
                        warn!(self.log, "conrod text unhandled");
                    }
                    _ => {}
                }
            }
            simple2d_renderer.render(&renderer, &mut frame, cmds);

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
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

widget_ids! {
    struct Ids { canvas, counter }
}
