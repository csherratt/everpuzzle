#![allow(unused_variables)]
use amethyst::ecs::prelude::WriteStorage;
use block_states::block_state::{change_state, BlockState};
use components::block::Block;
use components::playfield::stack::Stack;

pub struct Hang;
impl BlockState for Hang {
    fn enter(b: &mut Block) {
        b.counter = 10;
    }

    fn exit(b: &mut Block) {}
    fn execute(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {}

    fn counter_end(i: usize, stack: &Stack, blocks: &mut WriteStorage<'_, Block>) {
        change_state(blocks.get_mut(stack.from_i(i)).unwrap(), "FALL");
    }
}
