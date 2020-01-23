extern crate soez_core;

use soez_core::prelude::*;

struct SomeState;

impl State for SomeState {
    fn get_name(&self) -> &'static str { "SomeState" }
}

fn main() {
    let context = ContextBuilder::create(Box::from(SomeState))
        .with_name("context creation example")
        .with_author("me")
        .build();
    
    context.print_stuff();
}