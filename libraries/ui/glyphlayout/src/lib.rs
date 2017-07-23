extern crate rusttype;
extern crate unicode_normalization;

use rusttype::{Font, Scale, PositionedGlyph, Rect, Vector, point, vector};
use unicode_normalization::{UnicodeNormalization};

#[derive(Clone, PartialEq)]
pub enum AlignH { Left, Center, Right, }

pub fn layout_text<'a>(
    text: &str, font: &'a Font, text_size: f32, container: Rect<f32>, align: AlignH,
) -> Vec<PositionedGlyph<'a>> {
    let (glyphs, glyphs_size) = layout_text_line(text, font, text_size, container);

    match align {
        AlignH::Left => glyphs,
        AlignH::Center => reposition_center(glyphs, glyphs_size, container),
        AlignH::Right => reposition_right(glyphs, glyphs_size, container),
    }
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

fn reposition_center(
    mut glyphs: Vec<PositionedGlyph>, glyphs_size: Vector<f32>, container: Rect<f32>
) -> Vec<PositionedGlyph> {
    let half_container_size = container.width() * 0.5;
    let half_glyphs_size = glyphs_size.x * 0.5;

    for glyph in &mut glyphs {
        let mut position = glyph.position();
        position.x += half_container_size - half_glyphs_size;
        // TODO: Avoid this clone somehow
        *glyph = glyph.unpositioned().clone()
            .positioned(position);
    }

    glyphs
}

fn reposition_right(
    mut glyphs: Vec<PositionedGlyph>, text_size: Vector<f32>, container: Rect<f32>
) -> Vec<PositionedGlyph> {
    let container_size = container.width() * 0.5;
    let text_size = text_size.x * 0.5;

    for glyph in &mut glyphs {
        let mut position = glyph.position();
        position.x += container_size - text_size;
        // TODO: Avoid this clone somehow
        *glyph = glyph.unpositioned().clone()
            .positioned(position);
    }

    glyphs
}
