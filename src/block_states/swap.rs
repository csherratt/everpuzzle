#![allow(unused_variables)]
use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::block_state::{BlockState, change_state};
use systems::block_system::check_for_hang;

pub const SWAP_TIME: f32 = 5.0;

// animates movement of the block to a direction - either left or right
pub struct Swap;
impl BlockState for Swap {
    fn enter(b: &mut Block) {}
    fn exit(b: &mut Block) {}

    fn execute(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
		let b = blocks.get_mut(entities[i]).unwrap();

		b.offset.0 = b.move_dir * 16.0 + -b.move_dir * ease_out_quad(
			SWAP_TIME - b.counter as f32,
			0.0, 16.0,
			SWAP_TIME
		);
	}

    fn counter_end(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) {
		let can_fall = {
			check_for_hang(i, entities, blocks)
		};

		let b = blocks.get_mut(entities[i]).unwrap();
		if can_fall {
			change_state(b, "HANG");
		}
		else {
			b.state = "IDLE";
			b.offset.0 = 0.0;
		}
    }
}

fn ease_out_quad(t: f32, b: f32, c: f32, d: f32) -> f32 {
	let new = t / d;	
	-c * new * (new - 2.0) + b
}