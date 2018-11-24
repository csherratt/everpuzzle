#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};
use data::block_data::COLS;

// only detects if this block can fall and sets the state to hang
// resets chainable to false if this block cant fall
pub struct Idle;
impl BlockState for Idle {
    fn enter(b: &mut Block) {}
    fn exit(b: &mut Block) {}

    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
        // condition based on another block in a different lifetime
        let mut down_condition: bool = false;

        // check if is in vec boundary
        if i > COLS {
            let down = blocks.get_mut(entities[i - COLS]).unwrap();
            down_condition = down.is_empty() || down.state == "HANG";
        }

        // change the block to state if it isnt empty and the block below is empty / or falling
        let b = blocks.get_mut(entities[i]).unwrap();
        if !b.is_empty() && down_condition {
            change_state(b, "HANG");
        }
        else {
            b.chainable = false;
        }
    }

    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {}
}