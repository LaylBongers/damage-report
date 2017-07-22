extern crate rusttype;
extern crate unicode_normalization;

use rusttype::{Font, Scale, Point, point, PositionedGlyph};
use unicode_normalization::{UnicodeNormalization};

pub fn layout_text<'a>(
    text: &str, position: Point<f32>, font: &'a Font, text_size: f32
) -> (Vec<PositionedGlyph<'a>>, f32) {
    let mut positioned_glyphs = Vec::new();

    let scale = Scale::uniform(text_size);
    let v_metrics = font.v_metrics(scale);

    let mut caret = point(position.x, position.y + v_metrics.ascent);
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

    (positioned_glyphs, caret.x - position.x)
}
