use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::block::Block;
use data::{
    helpers::i2tuple,
    block_data::{
        BLOCKS, 
        COLS, 
        ROWS
    }
};

pub struct BlockSystem;

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem {  }
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
    
        // indent because of the (&mut blocks).join() not allowing
        // any other borrows afterwards
        {
            let mut search_blocks = (&mut blocks).join();
            // detect if any block can currently fall by looking for the
            // bottom blocks.kind, if so set a var to true
            for i in COLS..BLOCKS {
                let top_block = search_blocks.get_unchecked(i as u32).unwrap();
                let bottom_block = search_blocks.get_unchecked((i - COLS) as u32).unwrap();

                if bottom_block.kind == -1 {
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

            // look through all blocks and check for same colored block.kinds
            // then set boundaries for a block, save it in an option
            // then do comparisons on each block for each other 
            for i in 0..BLOCKS {
                let (x, y) = i2tuple(i);
                let b = search_blocks.get_unchecked(i as u32).unwrap();

                // closure so we can use inside variables that dont change
                // this returns a block that is within boundaries
                let mut get_block = | index, x_off, y_off | -> Option<&mut Block> {
                    if (x as usize) < COLS - 1 - x_off && (y as usize) < ROWS - 1 - y_off {
                        Some(search_blocks.get_unchecked(index as u32).unwrap())
                    }
                    else {
                        None
                    }
                };
                
                // example, we start at top left, search for two right, two bottom
                // 0  0  0
                // 0 -1 -1
                // 0 -1 -1
                // all nulls should match and get cleared next
                let mut right_neighbor = get_block(i + 1, 0, 0);
                let mut right_right_neighbor = get_block(i + 2, 1, 0);
                let mut top_neighbor = get_block(i + COLS, 0, 0);
                let mut top_top_neighbor = get_block(i + COLS * 2, 0, 1);

                b.check_similar_blocks(right_neighbor, right_right_neighbor); 
                b.check_similar_blocks(top_neighbor, top_top_neighbor); 
            }
        }
         
        // clears all blocks that have a should clear tag on it
        for block in (&mut blocks).join() {
            if block.should_clear {
                block.kind = -1;
                block.should_clear = false;
            }
        }
    }
}
