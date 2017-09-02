use cgmath::{Vector2};
use palette::pixel::{Srgb};
use screenmath::{Lrtb};

use style::{Style, Size, Position, SideH, SideV, SizeValue, color_highlight, color_active};
use {Element, ElementId, Ui, ElementBehavior};

/// A ribbon toolbar container. Useful as an alternative for traditional toolbars.
pub fn ribbon(parent: ElementId, ui: &mut Ui) -> ElementId {
    let ribbon_color = Srgb::new(0.18, 0.20, 0.21).into();
    let ribbon_style = Style {
        size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(104.0)),
        background_color: Some(ribbon_color),
        .. Style::new()
    };
    let mut ribbon = Element::new(ribbon_style.clone());
    ribbon.set_behavior(ElementBehavior::Clickable);
    let ribbon_id = ui.elements.add_child(ribbon, parent);

    let ribbon_buttons = Element::new(Style {
        position: Position::Relative(Vector2::new(0.0, 24.0), SideH::Left, SideV::Top),
        size: Size::new(SizeValue::Scale(1.0), SizeValue::Units(84.0)),
        .. Style::new()
    });
    let ribbon_buttons_id = ui.elements.add_child(ribbon_buttons, ribbon_id);

    ribbon_buttons_id
}

/// A clickable button that can be used inside a ribbon.
pub fn ribbon_buton(label: &str, parent: ElementId, ui: &mut Ui) -> ElementId {
    let ribbon_color = Srgb::new(0.18, 0.20, 0.21).into();
    let button_color = Srgb::new(0.53, 0.54, 0.52).into();

    let button_style = Style {
        margin: Lrtb::uniform(3.0),
        size: Size::units(60.0, 74.0),
        hover_color: Some(color_highlight(ribbon_color)),
        active_color: Some(color_active(ribbon_color)),
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

    let mut button = Element::new(button_style.clone());
    button.set_behavior(ElementBehavior::Clickable);
    let button_id = ui.elements.add_child(button, parent);

    let button_image = Element::new(button_image_style.clone());
    ui.elements.add_child(button_image, button_id);

    let mut button_text = Element::new(button_text_style.clone());
    button_text.set_text(label);
    ui.elements.add_child(button_text, button_id);

    button_id
}
