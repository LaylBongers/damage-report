use cgmath::{Vector2, Zero};
use palette::pixel::{Srgb};

use widget;
use style::{Style, Size, Position, SideH, SideV, Lrtb, SizeValue, FlowDirection};
use {Ui, Element, ElementId, ElementMode};

pub struct FileDialog {
    _shade_id: ElementId,
}

impl FileDialog {
    pub fn new(parent_id: ElementId, ui: &mut Ui) -> Self {
        // Transparent shade that blocks off the background, making this a modal dialog
        let mut shade = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Left, SideV::Top),
            size: Size::scale(1.0, 1.0),
            background_color: Some(Srgb::with_alpha(0.0, 0.0, 0.0, 0.5).into()),
            .. Style::new()
        });
        shade.mode = ElementMode::Clickable;
        let shade_id = ui.add_child(shade, parent_id);

        // Build up the actual dialog itself
        let dialog = Element::new(Style {
            position: Position::Relative(Vector2::zero(), SideH::Center, SideV::Center),
            size: Size::units(480.0, 240.0),
            padding: Lrtb::uniform(6.0),
            flow_direction: FlowDirection::Down,
            background_color: Some(Srgb::new(0.18, 0.20, 0.21).into()),
            .. Style::new()
        });
        let dialog_id = ui.add_child(dialog, shade_id);

        // Create a the submit and cancel buttons
        let buttons = Element::new(Style {
            // This position should counter-act the button's margins
            position: Position::Relative(Vector2::new(6.0, 6.0), SideH::Right, SideV::Bottom),
            // Button sizes + margins
            size: Size::units(90.0*2.0 + 6.0*3.0, 24.0 + 6.0*2.0),
            .. Style::new()
        });
        let buttons_id = ui.add_child(buttons, dialog_id);
        let _submit_button_id = widget::button("Save", buttons_id, ui);
        let _cancel_button_id = widget::button("Cancel", buttons_id, ui);

        // Styles for input fields
        let label_style = Style {
            size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(18.0)),
            margin: Lrtb::uniform(6.0),
            text_size: 18.0,
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            .. Style::new()
        };
        let textfield_style = Style {
            size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(18.0 + 12.0)),
            margin: Lrtb::uniform(6.0),
            padding: Lrtb::uniform(6.0),
            background_color: Some(Srgb::new(0.53, 0.54, 0.52).into()),
            text_size: 18.0,
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            .. Style::new()
        };

        // Add the directory field
        ui.add_child(Element::with_text("Directory", label_style.clone()), dialog_id);
        ui.add_child(Element::with_text("C:\\", textfield_style.clone()), dialog_id);

        // Add the file field
        ui.add_child(Element::with_text("File Name", label_style.clone()), dialog_id);
        ui.add_child(Element::with_text("my_map", textfield_style.clone()), dialog_id);

        FileDialog {
            _shade_id: shade_id,
        }
    }
}
