use amethyst::{
    utils::fps_counter::FPSCounter,
    core::timing::{duration_to_nanos, Time},
    ecs::*,
};

use basics::{
    block::Block,
};

pub struct FSMSystem;

impl<'a> System<'a> for FSMSystem {
    type SystemData = (
        WriteStorage<'a, FSM>, 
        WriteStorage<'a, Block>,
    );

    fn run(&mut self, (
        mut fsms,
        mut blocks,
        ): Self::SystemData) 
    {
        // update call
        for (fsm, block) in (&mut fsms, &mut blocks).join() {
            if fsm.counter > 0 {
                fsm.counter -= 1;
            }

            // update state
            match fsm.state {
                States::Idle => (),
                _ => ()
            }
        }
    }
}

impl FSMSystem {
    fn idle_update(b: &mut Block, stack: &BlockStack) {
         
    }
}
