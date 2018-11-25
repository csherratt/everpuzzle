use amethyst::ecs::*;
use basics::{
	block::Block,
	stack::Stack,
};
use block_states::block_state::change_state;

use data::{
	block_data::{
		BLOCKS,
		COLS,
		ROWS,
	},
	helpers::{tuple2i, i2tuple},
};

pub struct PlayfieldSystem {
	clear_queue: Vec<u32>,
	combo_counter: i32, 
	chain: u32,
	last_chain: u32,
}

impl Default for PlayfieldSystem {
	fn default() -> PlayfieldSystem {
		PlayfieldSystem {
			clear_queue: Vec::new(),
			combo_counter: 0,
			chain: 1,
			last_chain: 1,
		}
	}
}

impl<'a> System<'a> for PlayfieldSystem {
    type SystemData = (
		WriteStorage<'a, Block>,
		Read<'a, Stack>,
    );

    fn run(&mut self, (mut blocks, stack): Self::SystemData) {
		for x in 0..COLS {
			for y in 0..ROWS {
				for clear_block_id in check_clear(x, y, &stack.entities, &blocks) {
					if !self.clear_queue.contains(&clear_block_id) {
						self.clear_queue.push(clear_block_id);
					}
				}
			}
		}

		// if no clears were found, dont go through all
		let clear_size = self.clear_queue.len() as i32;
		if clear_size != 0 {
			self.combo_counter = 0;

			// animation times, TODO: get playfield level dependant times
			let flash: i32 = 44;
			let face: i32 = 10;
			let pop: i32 = 10;

			let all_time = flash + face + pop * clear_size;

			// set all animation times and general time it will take all blocks that are 
			// comboing to finish their animation
			for id in &self.clear_queue {
				let b = blocks.get_mut(stack.entities[*id as usize]).unwrap();
				let set_time = flash + face + pop * self.combo_counter;
				b.clear_time = set_time;
				self.combo_counter += 1;

				b.counter = all_time as u32;
				b.clear_start_counter = all_time;
				change_state(b, "CLEAR");
			}
		}
		 
		// clear the clear_queue if its not empty 
		if self.clear_queue.len() != 0 {
			println!("{:?}", self.clear_queue);
			self.clear_queue.clear();
		}
	}
}

// checks through eachs block right, right_right and up, up_up to see if they are performing a combo
// returns an array of block ids to identify them
fn check_clear(x: usize, y: usize, entities: &Vec<Entity>, blocks: &WriteStorage<'_, Block>) -> Vec<u32> {
	let mut checks: Vec<u32> = Vec::new();

	let r_rr = check_similar_block(x, y, 1, 0, entities, blocks);
	let u_uu = check_similar_block(x, y, 0, 1, entities, blocks);

	if let Some(mut right_vec) = r_rr {
		checks.append(&mut right_vec);
	}

	if let Some(mut up_vec) = u_uu {
		checks.append(&mut up_vec);
	}

	checks
}

// checks for similar blocks from the current block to 2 others
// checks if they all exist, are comboable, and also if their kinds match with the first
// returns an array of u32 ids of the blocks that are comboable or nothing
// to save on cpu -> not creating empty vecs
fn check_similar_block(
	x: usize, y: usize,
	x_offset: usize, y_offset: usize,
	entities: &Vec<Entity>, 
	blocks: &WriteStorage<'_, Block>
) -> Option<Vec<u32>> {
	let b1 = blocks.get(entities[tuple2i((x as f32, y as f32))]).unwrap();

	let check_boundary = |x: usize, y: usize| -> Option<&Block> {
		if x < COLS && y < ROWS {
			blocks.get(entities[tuple2i((x as f32, y as f32))])
		}
		else {
			None
		}
	};

	let b2 = check_boundary(x + x_offset, y + y_offset);
	let b3 = check_boundary(x + x_offset * 2, y + y_offset * 2);

	if b1.is_comboable() {
		if let Some(block2) = b2 {
			if let Some(block3) = b3 {
				if block2.is_comboable_with(b1.kind) && block3.is_comboable_with(b1.kind) {
					return Some(vec! [
						b1.id,
						block2.id,
						block3.id,
					])
				}
			}
		}
	}	

	// just return nothing to save up on cpu
	// we could just return an empty vec but since this happens around 72 * 2 times its expensive to do so
	None
}
