use calcium_rendering_simple2d::{Simple2DRenderer, RenderCommands};
use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoFrame};

pub struct VulkanoSimple2DRenderer {
}

impl VulkanoSimple2DRenderer {
    pub fn new() -> Self {
        VulkanoSimple2DRenderer {
        }
    }
}

impl Simple2DRenderer<VulkanoBackendTypes> for VulkanoSimple2DRenderer {
    fn render(&mut self, _frame: &mut VulkanoFrame, _commands: RenderCommands) {
    }
}
