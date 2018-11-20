use amethyst::{
    ecs::{
        *, 
        shred::FetchMut,
        storage::MaskedStorage,
        join::{Join, JoinIter},
    },
    renderer::*,
    core::Transform,
};

use std::cmp::max;
use basics::block::{Block, States};
use data::block_data::BLOCKS;

pub struct BlockSystem {
    clear_queue: Vec<u32>, // holds all unique ids of block entities that were matched
    combo_counter: u32,
    chain: u32,
    last_chain: u32,
    blocks_cleared: u32,
}

impl Default for BlockSystem {
    fn default() -> BlockSystem {
        BlockSystem {
            clear_queue: Vec::new(),
            combo_counter: 0,
            chain: 0,
            last_chain: 0,
            blocks_cleared: 0,
        }
    }
}

impl BlockSystem {
    fn any_chainable_exists(
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) -> bool {
        for i in 0..BLOCKS {
            let b = blocks.get_unchecked(i as u32).unwrap();

            if b.chainable {
                return true;
            }
        }

        return false;
    }
}

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Block>,
    );

    fn run(&mut self, (
            mut sprites, 
            mut transforms, 
            mut blocks,
            ): Self::SystemData)
    {
        // run all blocks state machines
        {
            let mut search_blocks = (&mut blocks).join();

            for i in 0..BLOCKS {
                let mut b = search_blocks.get_unchecked(i as u32).unwrap();
                b.update_state(&mut search_blocks);
            }
        }

        // start fresh with clearing blocks 
        {
            let mut search_blocks = (&mut blocks).join();

            for i in 0..BLOCKS {
                let b = search_blocks.get_unchecked(i as u32).unwrap();

                for clear_id in b.check_clear(&mut search_blocks) {
                    if !self.clear_queue.contains(&clear_id) {
                        self.clear_queue.push(clear_id);
                    }
                }
            }
            
            if self.clear_queue.len() != 0 {
                self.combo_counter = 0;

                // gather all animation times
                let flash = 44;
                let face = 15;
                let pop = 9;

                // sum them up
                let all_time = flash + face + pop * self.clear_queue.len();

                // check wehter any of the blocks in the stack were chainable
                let had_chainable: bool = BlockSystem::any_chainable_exists(&mut search_blocks);
                
                // increase the chain further
                if had_chainable {
                    self.chain += 1;
                    self.last_chain = max(self.chain, self.last_chain);
                }
                // otherwhise just reset chain back
                else {
                    self.chain = 1;
                }

                // go through all clear_ids and blocks, set their times
                for id in &self.clear_queue {
                    let b = search_blocks.get_unchecked(*id).unwrap();
                    let set_time = flash + face + pop * self.combo_counter as usize;
                    b.clear_time = set_time as i32;
                    self.combo_counter += 1; 

                    // set the time a block takes to clear relative to its pos
                    b.counter = all_time as i32;
                    b.clear_start_counter = all_time as i32;
                    b.change_state(States::Clear);
                }

                self.blocks_cleared += self.combo_counter;
                println!("highest_chain: {}, blocks_cleared: {}", self.chain, self.blocks_cleared);
                self.clear_queue.clear();
            }
        }

        // scale block if provided, position them by their size and given coordinate
        // set their sprite number by the block.kind
        for (sprite, block, transform) in (&mut sprites, &mut blocks, &mut transforms).join() {
            block.set_position(transform);
            block.kind_visible(sprite);
        }
    }
}
