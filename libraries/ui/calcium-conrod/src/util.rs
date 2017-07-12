use cgmath::{Vector4};
use palette::{Rgba};
use palette::pixel::{Srgb};

use conrod::{Color};

pub fn color_conrod_to_calcium(color: Color) -> Vector4<f32> {
    let c = color.to_rgb();
    let c = Srgb::with_alpha(c.0, c.1, c.2, c.3);
    let c: Rgba = c.into();
    Vector4::new(c.red, c.green, c.blue, c.alpha)
}
