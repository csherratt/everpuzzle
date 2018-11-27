#![allow(unused_variables)]
use amethyst::ecs::prelude::WriteStorage;
use block_states::block_state::{change_state, BlockState};
use components::block::Block;
use components::playfield::stack::Stack;
use systems::block_system::check_for_hang;

// only detects if this block can fall and sets the state to hang
// resets chainable to false if this block cant fall
pub struct Idle;
impl BlockState for Idle {
    fn enter(b: &mut Block) {}
    fn exit(b: &mut Block) {}

    fn execute(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {
        let can_hang: bool = { check_for_hang(i, stack, blocks) };

        // change the block to state if it isnt empty and the block below is empty / or falling
        let b = blocks.get_mut(stack.from_i(i)).unwrap();
        if can_hang {
            change_state(b, "HANG");
        } else {
            b.chainable = false;
        }
    }

    fn counter_end(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {}
}
