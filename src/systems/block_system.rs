use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::block::{Block, States};
use data::block_data::BLOCKS;

pub struct BlockSystem {
    clear_queue: Vec<u32>,
    combo_counter: u32,
}

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem {
            clear_queue: Vec::new(),
            combo_counter: 0,
        }
    }
}

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Block>,
        Entities<'a>,
    );

    fn run(&mut self, (
            mut sprites, 
            mut transforms, 
            mut blocks,
            entities
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

                let flash = 44;
                let face = 15;
                let pop = 9;

                let all_time = flash + face + pop * self.clear_queue.len();
                
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
            }

            if self.clear_queue.len() != 0 {
                println!("currently clearing: {:?}", self.clear_queue);
            }
            self.clear_queue.clear();
        }

        // scale block if provided, position them by their size and given coordinate
        // set their sprite number by the block.kind
        for (sprite, block, transform) in (&mut sprites, &mut blocks, &mut transforms).join() {
            block.set_position(transform);
            block.kind_visible(sprite);
        }
    }
}
