#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};

pub struct Hang;
impl BlockState for Hang {
    fn enter(b: &mut Block) {
        b.counter = 10;
    }

    fn exit(b: &mut Block) {}
    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {}

    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
        change_state(blocks.get_mut(entities[i]).unwrap(), "FALL");
    }
}