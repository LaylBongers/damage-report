extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate cgmath;
extern crate conrod;
#[macro_use]
extern crate slog;

use calcium_rendering::{BackendTypes, WindowRenderer};
use calcium_rendering_simple2d::{RenderCommands, Rectangle};
use cgmath::{Vector2, Vector4};
use conrod::{Ui};
use conrod::render::{PrimitiveWalker, PrimitiveKind};
use slog::{Logger};

pub struct ConrodRenderer {
}

impl ConrodRenderer {
    pub fn new(log: &Logger) -> Self {
        info!(log, "Creating conrod renderer");

        ConrodRenderer {
        }
    }

    pub fn draw_ui<T: BackendTypes>(
        &self, window: &T::WindowRenderer, ui: &mut Ui
    ) -> RenderCommands {
        let half_size: Vector2<i32> = window.size().cast() / 2;
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
                _ => {}
            }
        }

        cmds
    }
}
