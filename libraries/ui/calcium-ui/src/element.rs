use style::{Style};

pub struct Element {
    style: Style,
}

impl Element {
    pub fn new(style: Style) -> Self {
        Element {
            style,
        }
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn clicked(&self) -> bool {
        false
    }
}
