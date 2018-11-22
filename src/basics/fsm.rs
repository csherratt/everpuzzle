use basics::block::States;

pub struct FSM {
    pub state: States,
    pub counter: i32,
}

impl Default for FSM {
    fn default() -> FSM {
        FSM {
            state: States::Idle,
            counter: 0,
        }
    }
}

impl Component for FSM {
    type Storage = DenseVecStorage<Self>;
}


