use amethyst::{
    ecs::*,
    renderer::*,
    core::*,
    core::cgmath::Vector3
};

use basics::block::Block;

pub struct BlockSystem;

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem { }
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
    }
}
