use crate::prelude::*;

pub struct OnBoardingState;

impl State for OnBoardingState {
    fn get_name(&self) -> &'static str {
        "OnBoardingState"
    }
}