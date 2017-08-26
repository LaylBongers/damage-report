use cgmath::{Vector2, Point3};
use input::{Input};
use palette::pixel::{Srgb};
use rusttype::{FontCollection};

use calcium_game::{AverageDelta, delta_to_fps};
use calcium_rendering::{Renderer, WindowRenderer, Error};
use calcium_rendering_simple2d::{Simple2DRenderTarget, Simple2DRenderer};
use calcium_flowy::{FlowyRenderer};
use flowy::{Ui, Element, ElementId, widget};
use flowy::style::{Style, Position, Size, SideH, SideV};
use flowy::widget::{FileDialog};

use carpenter_model::{MapEditor};

pub struct UiView<R: Renderer> {
    ui_renderer: FlowyRenderer<R>,

    ui: Ui,
    save_as_id: ElementId,
    new_brush_id: ElementId,
    fps_id: ElementId,
    save_dialog: Option<FileDialog>,

    average_delta: AverageDelta,
}

impl<R: Renderer> UiView<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        let ui_renderer = FlowyRenderer::new(renderer)?;

        let mut ui = Ui::new();
        let root_id = ui.elements.root_id();

        // Load in a font
        let font = FontCollection::from_bytes(
            ::ttf_noto_sans::REGULAR
        ).into_font().unwrap();
        ui.fonts.push(font);

        // Create the top ribbon
        let ribbon_buttons_id = widget::ribbon(root_id, &mut ui);

        // Add a few buttons to the top ribbon
        let save_as_id = widget::ribbon_buton("Save As", ribbon_buttons_id, &mut ui);
        let _ = widget::ribbon_buton("Load", ribbon_buttons_id, &mut ui);
        let new_brush_id = widget::ribbon_buton("New Brush", ribbon_buttons_id, &mut ui);

        // Add a FPS label
        let fps = Element::new(Style {
            position: Position::Relative(Vector2::new(0.0, 0.0), SideH::Right, SideV::Top),
            size: Size::units(120.0, 14.0),
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            text_size: 14.0,
            .. Style::new()
        });
        let fps_id = ui.elements.add_child(fps, root_id);

        Ok(UiView {
            ui_renderer,

            ui,
            save_as_id,
            new_brush_id,
            fps_id,
            save_dialog: None,

            average_delta: AverageDelta::new(),
        })
    }

    pub fn handle_event(&mut self, event: &Input) {
        self.ui.handle_event(event);
    }

    pub fn update(&mut self, delta: f32, editor: &mut MapEditor) {
        self.ui.process_input_frame();
        self.average_delta.accumulate(delta);
        let root_id = self.ui.elements.root_id();

        {
            let fps = &mut self.ui.elements[self.fps_id];
            fps.set_text(format!("FPS: {}", delta_to_fps(self.average_delta.get())));
        }

        if self.ui.elements[self.save_as_id].clicked() {
            self.save_dialog = Some(widget::FileDialog::new(
                ::home::home_dir().unwrap_or("/".into()), root_id, &mut self.ui
            ));
        }

        if self.ui.elements[self.new_brush_id].clicked() {
            editor.new_brush(Point3::new(0.0, 0.0, 0.0));
            editor.new_brush(Point3::new(3.0, 0.0, 0.0));
            editor.new_brush(Point3::new(0.0, 0.0, 5.0));
        }

        if let Some(mut save_dialog) = self.save_dialog.take() {
            save_dialog.update(&mut self.ui);

            if save_dialog.submitted() {
                let target = save_dialog.selected_path();
                editor.set_save_target(target.clone());
            }

            // If it hasn't been closed yet, keep it
            if !save_dialog.closed() {
                self.save_dialog = Some(save_dialog);
            }
        }
    }

    pub fn render<SR: Simple2DRenderer<R>>(
        &mut self, frame: &mut R::Frame,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
        simple2d_renderer: &mut SR,
        simple2d_rendertarget: &mut Simple2DRenderTarget<R, SR>,
    ) -> Result<(), Error> {
        let ui_batches = self.ui_renderer.draw(
            &mut self.ui, window_renderer.size().cast(), renderer
        )?;

        simple2d_renderer.render(
            &ui_batches, simple2d_rendertarget,
            renderer, window_renderer, frame
        );

        Ok(())
    }

    pub fn cursor_over_ui(&self) -> bool {
        self.ui.cursor_active_element().is_some()
    }
}
