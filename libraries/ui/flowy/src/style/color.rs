use palette::{Rgba, Luma};

pub fn color_highlight(color: Rgba) -> Rgba {
    let luma: Luma = color.into();

    if luma.luma > 0.8 {
        Rgba::new(color.red - 0.2, color.green - 0.2, color.blue - 0.2, color.alpha)
    } else if luma.luma < 0.2 {
        Rgba::new(color.red + 0.2, color.green + 0.2, color.blue + 0.2, color.alpha)
    } else {
        Rgba::new(
            clampf32((1.0 - color.red) * 0.5 * color.red + color.red),
            clampf32((1.0 - color.green) * 0.1 * color.green + color.green),
            clampf32((1.0 - color.blue) * 0.1 * color.blue + color.blue),
            color.alpha,
        )
    }
}

pub fn color_active(color: Rgba) -> Rgba {
    let luma: Luma = color.into();

    if luma.luma > 0.8 {
        Rgba::new(color.red, color.green - 0.2, color.blue - 0.2, color.alpha)
    } else if luma.luma < 0.2 {
        Rgba::new(color.red + 0.4, color.green + 0.2, color.blue + 0.2, color.alpha)
    } else {
        Rgba::new(
            clampf32((1.0 - color.red) * 0.75 * color.red + color.red),
            clampf32((1.0 - color.green) * 0.25 * color.green + color.green),
            clampf32((1.0 - color.blue) * 0.25 * color.blue + color.blue),
            color.alpha,
        )
    }
}

fn clampf32(f: f32) -> f32 {
    if f < 0.0 { 0.0 } else if f > 1.0 { 1.0 } else { f }
}
