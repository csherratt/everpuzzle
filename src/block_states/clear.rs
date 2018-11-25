#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};

const FLASH_ANIM: [u32; 4] = [6, 6, 0, 0];
const FLASH_TIME: i32 = 44; 

pub struct Clear;
impl BlockState for Clear {
	// for safety of animating set the counter back
    fn enter(b: &mut Block) {
		b.anim_counter = 0
    }

    fn exit(b: &mut Block) {
		b.kind = -1;
		b.counter = 0;
		b.anim_offset = 0;

		// clear variable resets
		b.clearing = false;
		b.clear_counter = 0;
		b.clear_anim_counter = 0;
	}

	// just the animation part of the whole clearing
    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
		let b = blocks.get_mut(entities[i]).unwrap();

		// clear at the end of the animation
		let test = b.clear_time as i32 - b.clear_counter as i32;
		if test <= 0 && !b.clearing {
			// particles spawn
			b.clearing = true;
		}
		else {
			b.clear_counter += 1;
			b.clear_anim_counter += 1;
			
			// split animation in 2 parts
			if b.clear_anim_counter < FLASH_TIME {
				// flashy animation
				if b.anim_counter == 0 {
					b.anim_counter = 4;
				}
				else {
					b.anim_offset = FLASH_ANIM[b.anim_counter as usize];
				}
			}
			else {
				// just the face sprite
				b.anim_offset = 5;
			}
		}
	}

    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
		// todo

        change_state(blocks.get_mut(entities[i]).unwrap(), "IDLE");
    }
}