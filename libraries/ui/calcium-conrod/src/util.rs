use cgmath::{Vector4};
use palette::{Rgba};
use palette::pixel::{Srgb};

use conrod::{Color};
use conrod::color::{Rgba as Crgba};

pub fn color_conrod_to_calcium(color: Color) -> Vector4<f32> {
    color_conrod_rgba_to_calcium(color.to_rgb())
}

pub fn color_conrod_rgba_to_calcium(color: Crgba) -> Vector4<f32> {
    let c = Srgb::with_alpha(color.0, color.1, color.2, color.3);
    let c: Rgba = c.into();
    Vector4::new(c.red, c.green, c.blue, c.alpha)
}
