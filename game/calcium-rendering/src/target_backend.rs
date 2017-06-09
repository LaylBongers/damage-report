pub trait TargetBackend {
    type Frame;

    fn start_frame(&mut self) -> Self::Frame;
    fn finish_frame(&mut self, Self::Frame);
}
