pub struct VkVertex {
    pub v_position: [f32; 2],
    pub v_color: [f32; 4],
}

impl_vertex!(VkVertex, v_position, v_color);
