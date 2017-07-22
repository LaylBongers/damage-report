use palette::{Rgba, Luma};

#[derive(Clone, Debug, PartialEq)]
pub enum CursorBehavior {
    /// Doesn't listen to cursor events, but clicks and hovering get blocked by it.
    Block,
    /// Doesn't listen to cursor events, clicks and hovering go right through.
    PassThrough,
    /// Can be clicked, can change colors if hovered over or clicked.
    Clickable {
        hover: Option<Rgba>,
        hold: Option<Rgba>,
    },
}

impl CursorBehavior {
    pub fn clickable(hover: Option<Rgba>, hold: Option<Rgba>) -> Self {
        CursorBehavior::Clickable {
            hover,
            hold,
        }
    }

    pub fn clickable_autocolor(base: Rgba) -> Self {
        let luma: Luma = base.into();

        let highlighted = highlighted_for(luma, base);
        let hold = hold_for(luma, base);

        CursorBehavior::clickable(Some(highlighted), Some(hold))
    }
}

fn highlighted_for(luma: Luma, color: Rgba) -> Rgba {
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

fn hold_for(luma: Luma, color: Rgba) -> Rgba {
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
