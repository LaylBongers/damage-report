use cgmath::{Vector2};
use palette::pixel::{Srgb};

use style::{Style, Lrtb, Size, CursorBehavior, Position, SideH, SideV, SizeValue};
use {Element, ElementId, Ui};

pub fn ribbon(ui: &mut Ui, parent: ElementId) -> ElementId {
    let ribbon_color = Srgb::new(0.18, 0.20, 0.21).into();
    let ribbon_style = Style {
        size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(102.0)),
        background_color: Some(ribbon_color),
        cursor_behavior: CursorBehavior::Block,
        .. Style::new()
    };
    let ribbon = Element::new(ribbon_style.clone());
    let ribbon_id = ui.add_child(ribbon, parent);

    let ribbon_buttons = Element::new(Style {
        position: Position::Relative(Vector2::new(0.0, 22.0), SideH::Left, SideV::Top),
        size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(84.0)),
        .. Style::new()
    });
    let ribbon_buttons_id = ui.add_child(ribbon_buttons, ribbon_id);

    ribbon_buttons_id
}

pub fn ribbon_buton(label: &str, ui: &mut Ui, parent: ElementId) -> ElementId {
    let ribbon_color = Srgb::new(0.18, 0.20, 0.21).into();
    let button_color = Srgb::new(0.53, 0.54, 0.52).into();

    let button_style = Style {
        margin: Lrtb::uniform(3.0),
        size: Size::units(60.0, 74.0),
        cursor_behavior: CursorBehavior::clickable_autocolor(ribbon_color),
        .. Style::new()
    };
    let button_image_style = Style {
        size: Size::units(60.0, 60.0),
        background_color: Some(button_color),
        .. Style::new()
    };
    let button_text_style = Style {
        size: Size::units(60.0, 14.0),
        position: Position::Relative(Vector2::new(0.0, 0.0), SideH::Left, SideV::Bottom),
        text_size: 14.0,
        text_color: Srgb::new(1.0, 1.0, 1.0).into(),
        text_align: (SideH::Center, SideV::Top),
        .. Style::new()
    };

    let button = Element::new(button_style.clone());
    let button_id = ui.add_child(button, parent);

    let button_image = Element::new(button_image_style.clone());
    ui.add_child(button_image, button_id);

    let mut button_text = Element::new(button_text_style.clone());
    button_text.set_text(label);
    ui.add_child(button_text, button_id);

    button_id
}
