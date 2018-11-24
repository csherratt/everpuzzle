#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};
use data::block_data::{COLS, BLOCKS};

const LAND_ANIM: [u32; 10] = [2, 2, 2, 3, 3, 3, 4, 4, 4, 0];
const LAND_TIME: u32 = 10;

// STOPS THE BLOCK FROM BEING CHAINABLE after animating that is
//
// used for animating the land state 
// just sets sprite offset to the current animation frames
pub struct Land;
impl BlockState for Land {
    // set length of how long the fall will last
    fn enter(b: &mut Block) {
        b.counter = LAND_TIME;
        b.anim_counter = LAND_TIME;
    }

    // set anim to 0 for safety, blocks arent chainable once the land is finished
    // being chainable finally stops here!
    fn exit(b: &mut Block) {
        b.anim_offset = 0;
        b.chainable = false;
    }

    // simply animate
    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
        let b = blocks.get_mut(entities[i]).unwrap();
        print!("we even getting here?");
        b.anim_offset = LAND_ANIM[(LAND_TIME - b.anim_counter - 1) as usize];
    }

    // change to idle on default
    // if above isnt null and hanging, set the counter to the aboves counter
    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
        let mut above_hanging: bool = false;
        let mut above_counter: u32 = 0;

        if i < BLOCKS - COLS {
            let above = blocks.get(entities[i + COLS]).unwrap();
            above_hanging = above.state == "HANG";
            above_counter = above.counter;
        }
 
        let b = blocks.get_mut(entities[i]).unwrap();
        if above_hanging {
            change_state(b, "HANG"); 
            b.counter = above_counter;
        }
        else {
            change_state(b, "IDLE"); 
        }
    }
}