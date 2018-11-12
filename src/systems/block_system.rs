use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::block::Block;
use data::block_data::{BLOCKS, COLS};

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

            if let Some(num) = block.kind {
                sprite.sprite_number = num as usize * 9;
            } else {
                sprite.sprite_number = 8;
            }
        }

        let mut search_blocks = (&mut blocks).join();
        for i in 0..BLOCKS {
            let top_block = search_blocks.get_unchecked(i as u32).unwrap();

            if let Some(b) = top_block.neighbor {
                let mut bottom_block = blocks.get(b).expect("bottom block");

                if bottom_block.kind == None {
                    top_block.can_fall = true;
                }
            }
        }
        /*
        for block in (&mut blocks).join() {

            if let Some(b) = block.neighbor {
                let mut bottom_block = read_blocks.get(b).expect("bottom block");

                if bottom_block.kind == None {
                    block.can_fall = true;
                }
            }
        }*/

        /*
        for block in (&mut blocks).join() {
            if block.can_fall {
                if let Some(b) = block.neighbor {
                    let mut bottom_block = blocks.get_mut(b).expect("bottom block");

                    bottom_block.kind = block.kind;
                    block.kind = None;
                    block.can_fall = true;
                }
            }
        }*/
    }
}

