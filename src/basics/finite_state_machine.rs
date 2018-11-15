use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::collections::HashMap;
use std::boxed::Box;

pub trait State {
    fn execute(&self);
    fn exit(&self);
    fn enter(&self);
    fn counter_end(&self);
}

pub struct Idle {

    
}

impl Idle {
    fn new() -> Idle {
        Idle {

        }
    }
}

impl State for Idle {
    fn execute(&self) {

    }

    fn exit(&self) { }
    fn enter(&self) { }
    fn counter_end(&self) { }
}

pub struct Fall {

}

impl Fall {
    fn new() -> Fall {
        Fall {

        }
    }
}

impl State for Fall {
    fn execute(&self) {

    }

    fn exit(&self) { }
    fn enter(&self) { }
    fn counter_end(&self) { }
}


pub struct FiniteStateMachine {
    state: String,
    counter: i32,
    states: HashMap<String, Box<&dyn State>>
}

impl FiniteStateMachine {
    pub fn new() -> FiniteStateMachine {
        let mut states = HashMap::new();
        states.insert(String::from("IDLE"), Box::new(Idle::new()));

        FiniteStateMachine {
            state: String::from("IDLE"),
            counter: 0,
            states
        }
    }

    pub fn change_state(&mut self, new_state: String) {
        if self.state == new_state {
            return
        }

        self.states.get(&self.state).unwrap().exit();

        self.state = new_state;
        self.states.get(&self.state).unwrap().enter();
    }

    pub fn update(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }

        self.states.get(&self.state).unwrap().execute();

        if self.counter <= 0 {
            self.states.get(&self.state).unwrap().counter_end();
        }
    }
}

/*
impl Component for FiniteStateMachine {
    type Storage = DenseVecStorage<Self>;
}*/
