use raylib::RaylibHandle;

#[derive(Debug)]
pub struct RaylibContext {
    pub handle: RaylibHandle,
}

impl RaylibContext {
    pub fn new(handle: RaylibHandle) -> Self {
        Self { handle }
    }
}
