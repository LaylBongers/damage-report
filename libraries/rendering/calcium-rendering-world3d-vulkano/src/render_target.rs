use calcium_rendering_vulkano::{VulkanoTypes, VulkanoRenderer, VulkanoWindowRenderer};
use calcium_rendering_world3d::{World3DRenderTargetRaw};

use {VulkanoWorld3DTypes, VulkanoWorld3DRenderer};

pub struct VulkanoWorld3DRenderTargetRaw {
}

impl World3DRenderTargetRaw<VulkanoTypes, VulkanoWorld3DTypes> for VulkanoWorld3DRenderTargetRaw {
    fn new(
        _should_clear: bool,
        _renderer: &VulkanoRenderer, _window_renderer: &VulkanoWindowRenderer,
        _world3d_renderer: &VulkanoWorld3DRenderer,
    ) -> Self {
        VulkanoWorld3DRenderTargetRaw {
        }
    }
}
