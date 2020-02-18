use std::time::{Instant, Duration};
use crate::prelude::*;

pub struct BootLoadState {
    pub start_instant: Instant
}

impl Default for BootLoadState {
    fn default() -> Self {
        Self {
            start_instant: Instant::now()
        }
    }
}

impl State for BootLoadState {
    fn get_name(&self) -> &'static str {
        "BootLoadState"
    }

    fn on_update(&mut self) -> Transition {
        if Instant::now() - self.start_instant > Duration::from_secs(2) {
            Transition::Quit
        } else {
            Transition::None
        }
    }
}