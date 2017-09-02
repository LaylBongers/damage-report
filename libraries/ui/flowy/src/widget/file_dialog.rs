use std::path::{PathBuf};

use cgmath::{Vector2, Zero};
use palette::pixel::{Srgb};
use screenmath::{Lrtb};

use widget;
use style::{Style, Size, Position, SideH, SideV, SizeValue, FlowDirection, color_active};
use {Ui, Element, ElementId, ElementBehavior};

/// A dialog window that allows the user to select a file path.
pub struct FileDialog {
    shade_id: ElementId,
    directory_textfield_id: ElementId,
    filename_textfield_id: ElementId,
    cancel_button_id: ElementId,
    submit_button_id: ElementId,

    closed: bool,
    submitted: bool,
    path: PathBuf,
}

impl FileDialog {
    /// Creates a new file dialog widget and opens it.
    pub fn new(directory: PathBuf, parent_id: ElementId, ui: &mut Ui) -> Self {
        // Transparent shade that blocks off the background, making this a modal dialog
        let mut shade = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Left, SideV::Top),
            size: Size::scale(1.0, 1.0),
            background_color: Some(Srgb::with_alpha(0.0, 0.0, 0.0, 0.5).into()),
            .. Style::new()
        });
        shade.behavior = ElementBehavior::Clickable;
        let shade_id = ui.elements.add_child(shade, parent_id);

        // Build up the actual dialog itself
        let dialog = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Center, SideV::Center),
            size: Size::units(480.0, 240.0),
            padding: Lrtb::uniform(6.0),
            flow_direction: FlowDirection::Down,
            background_color: Some(Srgb::new(0.18, 0.20, 0.21).into()),
            .. Style::new()
        });
        let dialog_id = ui.elements.add_child(dialog, shade_id);

        // Styles for input fields
        let label_style = Style {
            size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(18.0)),
            margin: Lrtb::uniform(6.0),
            text_size: 18.0,
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            .. Style::new()
        };
        let textfield_color = Srgb::new(0.53, 0.54, 0.52).into();
        let textfield_style = Style {
            size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(18.0 + 12.0)),
            margin: Lrtb::uniform(6.0),
            padding: Lrtb::uniform(6.0),

            background_color: Some(textfield_color),
            hover_color: Some(color_active(textfield_color)),
            active_color: Some(color_active(textfield_color)),

            text_size: 18.0,
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            .. Style::new()
        };

        // Add the directory field
        ui.elements.add_child(Element::with_text("Directory", label_style.clone()), dialog_id);
        let mut directory_textfield = Element::with_text(
            directory.to_str().unwrap(), textfield_style.clone()
        );
        directory_textfield.behavior = ElementBehavior::TextField;
        let directory_textfield_id = ui.elements.add_child(directory_textfield, dialog_id);

        // Add the file field
        ui.elements.add_child(Element::with_text("File Name", label_style.clone()), dialog_id);
        let mut filename_textfield = Element::with_text("my_map.carpenter", textfield_style.clone());
        filename_textfield.behavior = ElementBehavior::TextField;
        let filename_textfield_id = ui.elements.add_child(filename_textfield, dialog_id);

        // Create the submit and cancel buttons
        let buttons = Element::new(Style {
            // This position should counter-act the buttons' margins
            position: Position::Relative(Vector2::new(6.0, 6.0), SideH::Right, SideV::Bottom),
            // Button sizes + margins TODO: Auto-Size
            size: Size::units(90.0*2.0 + 6.0*3.0, 24.0 + 6.0*2.0),
            .. Style::new()
        });
        let buttons_id = ui.elements.add_child(buttons, dialog_id);
        let submit_button_id = widget::button("Save", buttons_id, ui);
        let cancel_button_id = widget::button("Cancel", buttons_id, ui);

        FileDialog {
            shade_id,
            directory_textfield_id,
            filename_textfield_id,
            cancel_button_id,
            submit_button_id,

            closed: false,
            submitted: false,
            path: PathBuf::new(),
        }
    }

    /// Returns true if this file dialog has been closed.
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Returns true if this file dialog has a submitted path.
    pub fn submitted(&self) -> bool {
        self.submitted
    }

    /// Returns the path that has been selected.
    pub fn selected_path(&self) -> &PathBuf {
        &self.path
    }

    /// Updates this file dialog's state, checks updates on internal widgets.
    pub fn update(&mut self, ui: &mut Ui) {
        if self.closed { return }

        if ui.elements[self.cancel_button_id].clicked() {
            self.close(ui);
            return
        }

        if ui.elements[self.submit_button_id].clicked() {
            {
                let directory_str = ui.elements[self.directory_textfield_id].text();
                let filename_str = ui.elements[self.filename_textfield_id].text();

                let mut path = PathBuf::from(directory_str);
                path.push(filename_str);
                self.path = path;
            }

            self.submitted = true;
            self.close(ui);
            return
        }
    }

    /// Closes this file dialog.
    pub fn close(&mut self, ui: &mut Ui) {
        ui.elements.remove(self.shade_id);
        self.closed = true;
    }
}
