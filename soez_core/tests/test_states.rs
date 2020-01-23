extern crate soez_core;

use std::collections::hash_map::HashMap;
use soez_core::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Counted {
    Pause,
    Exit,
    Start,
    Resume,
    Update
}

#[derive(Debug)]
struct StateMock {
    pub name: &'static str,
    pub transition: Transition,
    pub counts: HashMap<Counted, usize>,
}

impl State for StateMock {
    fn get_name(&self) -> &'static str { self.name }

    fn on_exit(&mut self) {
        *self.counts.entry(Counted::Exit).or_insert(0) += 1;
    }

    fn on_pause(&mut self) {
        *self.counts.entry(Counted::Pause).or_insert(0) += 1;
    }

    fn on_resume(&mut self) -> Transition {
        *self.counts.entry(Counted::Resume).or_insert(0) += 1;
        std::mem::replace(&mut self.transition, Transition::None)
    }

    fn on_start(&mut self) -> Transition {
        *self.counts.entry(Counted::Start).or_insert(0) += 1;
        std::mem::replace(&mut self.transition, Transition::None)
    }

    fn on_update(&mut self) -> Transition {
        *self.counts.entry(Counted::Update).or_insert(0) += 1;
        std::mem::replace(&mut self.transition, Transition::None)
    }
}

#[test]
fn initialize_pushdown() {
    Pushdown::new(Box::from(StateMock {
        name: "root",
        counts: HashMap::default(),
        transition: Transition::Push(Box::from(StateMock {
            name: "root.1",
            counts: HashMap::default(),
            transition: Transition::Pop
        }))
    }));
}

#[test]
fn check_single_pop() {
    let mut pd = Pushdown::new(Box::from(StateMock {
        name: "root",
        counts: HashMap::default(),
        transition: Transition::Pop
    }));

    let transition = pd.peek_mut().on_update();

    assert_eq!(true, match transition {
        Transition::Pop => true,
        _ => false,
    });
}