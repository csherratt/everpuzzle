#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};
use data::block_data::COLS;
use systems::block_system::check_for_hang;

// only detects if this block can fall and sets the state to hang
// resets chainable to false if this block cant fall
pub struct Idle;
impl BlockState for Idle {
    fn enter(b: &mut Block) {}
    fn exit(b: &mut Block) {}

    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
        let can_hang: bool = {
            check_for_hang(i, entities, blocks)
        };

        // change the block to state if it isnt empty and the block below is empty / or falling
        let b = blocks.get_mut(entities[i]).unwrap();
        if can_hang {
            change_state(b, "HANG");
        }
        else {
            b.chainable = false;
        }
    }

    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {}
}