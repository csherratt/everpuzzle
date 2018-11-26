use amethyst::ecs::*;
use components::{
	block::Block,
	playfield::stack::Stack,
	cursor::Cursor,
	playfield::{
		playfield_clear::PlayfieldClear,
		playfield_push::PlayfieldPush,
	},
};
use block_states::block_state::change_state;
use data::block_data::{BLOCKS, COLS, ROWS};
use std::cmp::max;

pub struct PlayfieldSystem;

impl<'a> System<'a> for PlayfieldSystem {
    type SystemData = (
		WriteStorage<'a, PlayfieldClear>,
		WriteStorage<'a, PlayfieldPush>,
		WriteStorage<'a, Block>,
		ReadStorage<'a, Stack>,
		WriteStorage<'a, Cursor>,
		Entities<'a>,
    );

    fn run(&mut self, (
		mut playfield_clears,
		mut playfield_pushes,
		mut blocks, 
		stacks, 
		mut cursors,
		entities,
	): Self::SystemData) {
		// playfield push info / push animation WIP
		for (entity, stack) in (&entities, &stacks).join() {
			{
				// store info in p_push
				let mut p_push = playfield_pushes.get_mut(entity).unwrap();
				p_push.any_clears = check_blocks_clearing(&stack, &blocks);
				p_push.any_top_blocks = check_blocks_at_top(&stack, &blocks); 
			}

			{
				// actually offset things based on time
				visual_offset(
					playfield_pushes.get_mut(entity).unwrap(), 
					&stack, 
					&mut blocks, 
					cursors.get_mut(stack.cursor_entity).unwrap(),
				);
			}
		}
	
		// block clear detection
		// counts the amount of clears each frame, passes them uniquely to an array holding their ids
		// sets a lot of playfield_clear values and then sets the blocks to animate with given times
		for (p_clear, p_push, stack) in (&mut playfield_clears, &mut playfield_pushes, &stacks).join() {
			for x in 0..COLS {
				for y in 0..ROWS {
					for clear_block_id in check_clear(x, y, &stack, &blocks) {
						if !p_clear.clear_queue.contains(&clear_block_id) {
							p_clear.clear_queue.push(clear_block_id);
						}
					}
				}
			}

			// if no clears were found, dont go through all
			let clear_size = p_clear.clear_queue.len() as u32;
			if clear_size != 0 {
				p_clear.combo_counter = 0;

				// animation times, TODO: get playfield level dependant times
				let flash: u32 = 44;
				let face: u32 = 10;
				let pop: u32 = 10;

				let all_time: u32 = flash + face + pop * clear_size;

				let had_chainable: bool = any_chainable_exists(&p_clear.clear_queue, stack, &blocks);

				// max the chain and save data in a last chain
				if had_chainable {
					p_clear.chain += 1;
					p_clear.last_chain = max(p_clear.chain, p_clear.last_chain);
				}
				// otherwhise reset the chain
				else {
					p_clear.chain = 1;
				}

				// set all animation times and general time it will take all blocks that are 
				// comboing to finish their animation
				for id in &p_clear.clear_queue {
					let b = blocks.get_mut(stack.from_i(*id as usize)).unwrap();
					let set_time = flash + face + pop * p_clear.combo_counter;
					b.clear_time = set_time as i32;
					p_clear.combo_counter += 1;

					b.counter = all_time;
					b.clear_start_counter = all_time as i32;
					change_state(b, "CLEAR");
				}

				// clear the clear_queue if its not empty 
				p_clear.blocks_cleared += p_clear.combo_counter;
				p_clear.clear_queue.clear();
				println!("chain: {}, combo: {}, blocks_cleared: {}", p_clear.chain, p_clear.combo_counter, p_clear.blocks_cleared);
			}
		}
	}
}

// returns true when any block was found that is currently in clear state
fn check_blocks_clearing(stack: &Stack, blocks: &WriteStorage<'_, Block>) -> bool {
	for i in 0..BLOCKS {
		let b = blocks.get(stack.from_i(i)).unwrap();

		if b.state == "CLEAR" {// or garbage clear
			return true;
		}
	}

	return false;
}

// returns true if any "real" block is at the top of the grid
fn check_blocks_at_top(stack: &Stack, blocks: &WriteStorage<'_, Block>) -> bool {
	for x in 0..COLS {
		let b = blocks.get(stack.from_xy(x, ROWS - 1)).unwrap();

		if b.kind != -1 && b.state == "IDLE" { // or garbage 
			return true;
		}
	}

	return false;
}

fn visual_offset(
	p_push: &mut PlayfieldPush,
	stack: &Stack,	
	blocks: &mut WriteStorage<'_, Block>,
	cursor: &mut Cursor,
) {
	// if any cursor signal comes through do smooth increase thats faster and stops
	if p_push.signal_raise {
		p_push.smooth_raise = true;
	}

	// stop any raise, even smooth call
	if p_push.any_clears || p_push.any_top_blocks {
		p_push.smooth_raise = false; // deletes all smooth_raise signals
		return;
	}

	// if anything blocks raise by setting its time all raise stops until it counts down
	// used to block the amount of time it takes until another raise triggers
	if p_push.raised_blocked_counter > 0 {
		p_push.raised_blocked_counter -= 1; 
		p_push.smooth_raise = false; // deletes all smooth_raise signals 
		return;
	}

	// until counter is at 16 (the block sprite size)
	if p_push.offset_counter > 16.0 {
		// reset all offsets and reset smoothing
		p_push.offset_counter = 0.0; 
		set_visual_offsets(0.0, stack, blocks, cursor);
		p_push.smooth_raise = false;
		p_push.raised_blocked_counter = 5; // TODO: GET TIME FROM FILE
	}
	else {
		// if smooth - increase faster
		if p_push.smooth_raise {
			p_push.offset_counter += 4.0;
		}
		// else slowly increase
		else {
			p_push.offset_counter += 0.025; // TODO: TIMES LEVEL DEPENDANT
		}

		set_visual_offsets(p_push.offset_counter, stack, blocks, cursor);
	}
}	

fn set_visual_offsets(
	value: f32, 
	stack: &Stack,
	blocks: &mut WriteStorage<'_, Block>,
	cursor: &mut Cursor,
	) {
	for i in 0..BLOCKS {
		blocks.get_mut(stack.from_i(i)).unwrap().offset.1 = value;
	}

	cursor.offset.1 = value;
}

// checks through eachs block right, right_right and up, up_up to see if they are performing a combo
// returns an array of block ids to identify them
fn check_clear(x: usize, y: usize, stack: &Stack, blocks: &WriteStorage<'_, Block>) -> Vec<u32> {
	let mut checks: Vec<u32> = Vec::new();

	let r_rr = check_similar_block(x, y, 1, 0, stack, blocks);
	let u_uu = check_similar_block(x, y, 0, 1, stack, blocks);

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
	stack: &Stack, 
	blocks: &WriteStorage<'_, Block>
) -> Option<Vec<u32>> {
	let b1 = blocks.get(stack.from_xy(x, y)).unwrap();

	let check_boundary = |x: usize, y: usize| -> Option<&Block> {
		if x < COLS && y < ROWS {
			blocks.get(stack.from_xy(x, y))
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

fn any_chainable_exists(
	clear_ids: &Vec<u32>,
	stack: &Stack, 
	blocks: &WriteStorage<'_, Block>,
	) -> bool {
	for id in clear_ids {
		if blocks.get(stack.from_i(*id as usize)).unwrap().chainable {
			return true;
		}
	}

	return false;
}