use serde::Deserialize;
use specs::prelude::*;

#[derive(Debug, Deserialize)]
pub struct SingleF32 {
    pub value: f32,
}

impl Component for SingleF32 {
    type Storage = VecStorage<Self>;
}
