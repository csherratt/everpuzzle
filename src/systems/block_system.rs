use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::block::Block;
use data::block_data::BLOCKS;

pub struct BlockSystem;

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
        {
            // update all state machines on each block
            let mut search_blocks = (&mut blocks).join();

            for i in 0..BLOCKS {
                let mut b = search_blocks.get_unchecked(i as u32).unwrap();
                b.update_state(&mut search_blocks);
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
