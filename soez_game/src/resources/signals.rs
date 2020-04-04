#[derive(Debug)]
pub struct Signals {
    pub pending_quit: bool,
}

impl Default for Signals {
    fn default() -> Self {
        Self {
            pending_quit: false,
        }
    }
}
