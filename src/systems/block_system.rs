use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::block::Block;
use data::{
    helpers::tuple2i,
    block_data::{
        BLOCKS, 
        COLS, 
        ROWS
    }
};

pub struct BlockSystem {

}

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem { 
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
            mut blocks,
            ): Self::SystemData)
    {
        // scale block if provided, position them by their size and given coordinate
        // set their sprite number by the block.kind
        for (sprite, block, transform) in (&mut sprites, &mut blocks, &mut transforms).join() {
            block.set_position(transform);

            if block.kind != -1 {
                sprite.sprite_number = block.kind as usize * 9;
            }
            else {
                sprite.sprite_number = 8;
            }
        }

        let mut search_blocks = (&mut blocks).join();
        // detect if any block can currently fall by looking for the
        // bottom blocks.kind, if so set a var to true
        for i in COLS..BLOCKS {
            let top_block = search_blocks.get_unchecked(i as u32).unwrap();
            let bottom_block = search_blocks.get_unchecked((i - COLS) as u32).unwrap();

            if bottom_block.kind != -1 {
                top_block.can_fall = true;
            }
        }

        // if any top blocks can fall, switch kinds with bottom
        // since bottom was always none - top can be none
        for i in COLS..BLOCKS {
            let top_block = search_blocks.get_unchecked(i as u32).unwrap();
            let bottom_block = search_blocks.get_unchecked((i - COLS) as u32).unwrap();

            if top_block.can_fall {
                bottom_block.kind = top_block.kind;
                top_block.kind = -1;
                top_block.can_fall = false;
            }
        }

        let search_color = 0;
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = tuple2i((x as f32, y as f32));

                let b1 = search_blocks.get_unchecked(i as u32).unwrap();
                let mut b2 = None;
                let mut b3 = None;
    
                if x < COLS - 1 {
                    b2 = Some(search_blocks.get_unchecked((i + 1) as u32).unwrap());
                }

                if x < COLS - 2 {
                    b3 = Some(search_blocks.get_unchecked((i + 2) as u32).unwrap());
                }

                b1.check_similar_blocks(b2, b3); 
           }
        }
    }
}

