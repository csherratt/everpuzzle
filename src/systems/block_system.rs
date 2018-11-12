use amethyst::{
    ecs::*,
    renderer::*,
    core::*,
    core::cgmath::Vector3
};

use basics::block::Block;
use data::block_data::{BLOCKS, COLS};
use std::rc::Rc;

pub struct BlockSystem {
    assign_neighbors: bool    
}

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem { 
            assign_neighbors: true
        }
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
            mut blocks): Self::SystemData)
    {
        // scale block if provided, position them by their size and given coordinate
        // set their sprite number by the block.kind
        for (sprite, block, transform) in (&mut sprites, &mut blocks, &mut transforms).join() {
            block.set_position(transform);

            if let Some(num) = block.kind {
                sprite.sprite_number = num as usize * 9;
            } else {
                sprite.sprite_number = 8;
            }
        }

        let mut search_blocks = (&mut blocks).join();

        // detect if any block can currently fall by looking for the
        // bottom blocks.kind, if so set a var to true
        for i in 0..BLOCKS - COLS {
            let bottom_block = search_blocks.get_unchecked(i as u32).unwrap();
            let top_block = search_blocks.get_unchecked((i + COLS) as u32).unwrap();

            if bottom_block.kind == None {
                top_block.can_fall = true;
            } 
        }

        // if any top block can fall, switch kinds with bottom
        // since bottom was always none - top can be none afterwards
        for i in 0..BLOCKS - COLS {
            let bottom_block = search_blocks.get_unchecked(i as u32).unwrap();
            let top_block = search_blocks.get_unchecked((i + COLS) as u32).unwrap();

            if top_block.can_fall {
                bottom_block.kind = top_block.kind;
                top_block.kind = None;
                top_block.can_fall = false;
            }
        }
    }
}

