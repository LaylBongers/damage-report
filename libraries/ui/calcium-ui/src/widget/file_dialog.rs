use cgmath::{Vector2, Zero};
use palette::pixel::{Srgb};

use style::{Style, CursorBehavior, Size, Position, SideH, SideV};
use {Ui, Element, ElementId};

pub struct FileDialog {
    _shade_id: ElementId,
}

impl FileDialog {
    pub fn new(ui: &mut Ui, root_id: ElementId) -> Self {
        // Transparent shade that blocks off the background, making this a modal dialog
        let shade = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Left, SideV::Top),
            size: Size::scale(1.0, 1.0),
            background_color: Some(Srgb::with_alpha(0.0, 0.0, 0.0, 0.5).into()),
            cursor_behavior: CursorBehavior::Block,
            .. Style::new()
        });
        let shade_id = ui.add_child(shade, root_id);

        // Build up the actual dialog itself
        let dialog = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Center, SideV::Center),
            size: Size::units(480.0, 240.0),
            background_color: Some(Srgb::new(0.18, 0.20, 0.21).into()),
            .. Style::new()
        });
        let dialog_id = ui.add_child(dialog, shade_id);

        // Set up the generic button style
        let button_color = Srgb::new(0.53, 0.54, 0.52).into();
        let button_style = Style {
            position: Position::Relative(Vector2::new(-6.0, -6.0), SideH::Right, SideV::Bottom),
            size: Size::units(90.0, 24.0),
            background_color: Some(button_color),
            text_size: 24.0,
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            text_align: (SideH::Center, SideV::Top),
            cursor_behavior: CursorBehavior::clickable_autocolor(button_color),
            .. Style::new()
        };

        // Create a submit button
        let mut submit_style = button_style.clone();
        submit_style.position = Position::Relative(
            Vector2::new(-6.0 - 6.0 - 90.0, -6.0), SideH::Right, SideV::Bottom
        );
        let mut submit_button = Element::new(submit_style);
        submit_button.set_text("Save");
        ui.add_child(submit_button, dialog_id);

        // Create a cancel/close button
        let mut cancel_button = Element::new(button_style);
        cancel_button.set_text("Cancel");
        ui.add_child(cancel_button, dialog_id);

        FileDialog {
            _shade_id: shade_id,
        }
    }
}
