pub trait Window {
    fn handle_events(&mut self) -> bool;
}
