
// TODO: Completely remove this file once everything's moved to different places


/*impl RenderBackend<VulkanoBackendTypes> for VulkanoRenderSystem {
    fn start_frame(&mut self) -> VulkanoFrame {
        self.target_swapchain.clean_old_submissions();

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (framebuffer, image_num, mut future) = self.target_swapchain.start_frame();

        // If we have any images to load, we need to submit another buffer before anything else
        if self.queued_image_copies.len() != 0 {
            // Create a command buffer to upload the textures with
            let mut image_copy_buffer_builder = AutoCommandBufferBuilder::new(
                self.device.clone(), self.graphics_queue.family()
            ).unwrap();

            // Add any textures we need to upload to the command buffer
            while let Some(val) = self.queued_image_copies.pop() {
                // Add the copy to the buffer
                image_copy_buffer_builder = image_copy_buffer_builder
                    .copy_buffer_to_image(val.0, val.1)
                    .unwrap();
            }

            // Add the command buffer to the future so it will be executed
            let image_copy_buffer = image_copy_buffer_builder.build().unwrap();
            future = Box::new(future
                .then_execute(self.graphics_queue.clone(), image_copy_buffer).unwrap()
            );
        }

        VulkanoFrame {
            framebuffer,
            image_num,
            future: Some(future),
        }
    }

    fn finish_frame(&mut self, mut frame: VulkanoFrame) {
        self.target_swapchain.finish_frame(
            frame.future.take().unwrap(), self.graphics_queue.clone(), frame.image_num
        );
    }
}*/
