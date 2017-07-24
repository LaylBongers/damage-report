extern crate rusttype;
extern crate unicode_normalization;

use rusttype::{Font, Scale, PositionedGlyph, Rect, Vector, Point, point, vector};
use unicode_normalization::{UnicodeNormalization};

#[derive(Clone, PartialEq)]
pub enum AlignH { Left, Center, Right, }

#[derive(Clone, PartialEq)]
pub enum AlignV { Top, Center, Bottom, }

pub fn layout_text<'a>(
    text: &str, font: &'a Font, text_size: f32, container: Rect<f32>, align: (AlignH, AlignV),
) -> Vec<PositionedGlyph<'a>> {
    let (mut glyphs, glyphs_size) = layout_text_line(text, font, text_size, container);

    let container_size = rect_size(container);
    let half_container_size = container_size * 0.5;
    let half_glyphs_size = glyphs_size * 0.5;

    // Align horizontally
    glyphs = match align.0 {
        AlignH::Left => glyphs,
        AlignH::Center => reposition(glyphs, |p| point(
            p.x + half_container_size.x - half_glyphs_size.x, p.y
        )),
        AlignH::Right => reposition(glyphs, |p| point(
            p.x + container_size.x - glyphs_size.x, p.y
        )),
    };

    // Align vertically
    glyphs = match align.1 {
        AlignV::Top => glyphs,
        AlignV::Center => reposition(glyphs, |p| point(
            p.x, p.y + half_container_size.y - half_glyphs_size.y
        )),
        AlignV::Bottom => reposition(glyphs, |p| point(
            p.x, p.y + container_size.y - glyphs_size.y
        )),
    };

    glyphs
}

fn rect_size(rect: Rect<f32>) -> Vector<f32> {
    vector(rect.width(), rect.height())
}

fn layout_text_line<'a>(
    text: &str, font: &'a Font, text_size: f32, container: Rect<f32>,
) -> (Vec<PositionedGlyph<'a>>, Vector<f32>) {
    let mut positioned_glyphs = Vec::new();

    let scale = Scale::uniform(text_size);
    let v_metrics = font.v_metrics(scale);
    //let new_line_height = v_metrics.ascent + -v_metrics.descent + v_metrics.line_gap;

    let mut caret = point(container.min.x, container.min.y + v_metrics.ascent);
    let mut last_glyph_id = None;

    // Convert the text to positioned glyphs
    // Normalizing to "Normalized Form C", reduces mojibake
    for c in text.nfc() {
        // Skip control characters in single-line drawing
        if c.is_control() {
            continue;
        }

        // Look up the glyph for this character
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };

        // Add the kerning needed for the last glyph next to this glyph
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());

        // Position the glyph for this character
        let glyph = base_glyph.scaled(scale).positioned(caret);
        /*if let Some(bb) = glyph.pixel_bounding_box() { TODO: Multi-line support
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }*/
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        positioned_glyphs.push(glyph);
    }

    (positioned_glyphs, vector(
        caret.x - container.min.x,
        caret.y + -v_metrics.descent - container.min.y
    ))
}

fn reposition<F: Fn(Point<f32>) -> Point<f32>>(
    mut glyphs: Vec<PositionedGlyph>, positioner: F
) -> Vec<PositionedGlyph> {
    for glyph in &mut glyphs {
        let position = positioner(glyph.position());
        *glyph = glyph.unpositioned().clone()
            .positioned(position);
    }

    glyphs
}
