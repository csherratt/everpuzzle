#![allow(unused_variables)]
use amethyst::ecs::prelude::WriteStorage;
use block_states::block_state::{change_state, BlockState};
use components::block::Block;
use components::playfield::stack::Stack;
use data::block_data::COLS;

// falls to one block below IN 1 FRAME
// sets the block below to this current one
// resets this blocks data to default
pub struct Fall;
impl BlockState for Fall {
    fn enter(b: &mut Block) {}
    fn exit(b: &mut Block) {}

    fn execute(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {
        let mut is_empty: bool = false;
        let mut state_hang: bool = false;
        let mut down_counter: u32 = 0;

        // if in boundary for down blocks to exist
        if i > COLS {
            let down = blocks.get_mut(stack[i - COLS]).unwrap();
            is_empty = down.is_empty();
            state_hang = down.state == "HANG";
            down_counter = down.counter;
        } else {
            let b = blocks.get_mut(stack[i]).unwrap();
            b.state = "IDLE";
            return;
        }

        if is_empty {
            // store data from the current to a temp
            let temp_block = *blocks.get(stack[i]).unwrap();

            // store data into the down block
            blocks
                .get_mut(stack[i - COLS])
                .unwrap()
                .set_properties(temp_block);

            // reset data in the current one to default
            blocks.get_mut(stack[i]).unwrap().reset();
        } else if state_hang {
            let b = blocks.get_mut(stack[i]).unwrap();
            b.state = "HANG";
            b.counter = down_counter;
        } else {
            change_state(blocks.get_mut(stack[i]).unwrap(), "LAND");
        }
    }

    fn counter_end(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {}
}
