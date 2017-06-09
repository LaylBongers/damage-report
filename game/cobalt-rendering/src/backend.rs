pub trait Backend {
    type Frame;

    fn start_frame(&mut self) -> Self::Frame;
    fn finish_frame(&mut self, Self::Frame);
}
