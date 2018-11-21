use amethyst::{
    ecs::{
        *, 
        join::{Join, JoinIter},
    },
    renderer::*,
    core::Transform,
};

use basics::block::{Block, States};
use game_modes::game_mode::BlockStack;
use data::block_data::BLOCKS;

pub struct BlockSystem {

}

impl Default for BlockSystem {
    fn default() -> BlockSystem {
        BlockSystem {

        }
    }
}

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Block>,
        Entities<'a>,
        Read<'a, BlockStack>,
    );

    fn run(&mut self, (
            mut sprites, 
            mut transforms, 
            mut blocks,
            entities,
            stack
            ): Self::SystemData)
    {
        // translation
        for (block, transform) in (&blocks, &mut transforms).join() {
            transform.translation.x = block.x as f32 * transform.scale.x * 16.0;
            transform.translation.y = block.y as f32 * transform.scale.y * 16.0;
        }

        for (block, sprite) in (&blocks, &mut sprites).join() {
            BlockSystem::update_sprites(block, sprite);
        }

        let mut join_blocks = (&mut blocks).join();
        for e in &stack.entities {
            let top = join_blocks.get(*e, &entities).unwrap();
            
            if let Some(bottom_entity) = top.down {
                let bot = join_blocks.get(bottom_entity, &entities).unwrap();

                if bot.kind == -1 {
                    top.can_fall = true;
                }
            }
        }

        for e in &stack.entities {
            let top = join_blocks.get(*e, &entities).unwrap();
            
            if let Some(bottom_entity) = top.down {
                let bot = join_blocks.get(bottom_entity, &entities).unwrap();

                if top.can_fall {
                    bot.kind = top.kind;
                    top.kind = -1;
                }
            }
        }
    }
}

impl BlockSystem {
    // changes the current sprite
    fn update_sprites(block: &Block, sprite: &mut SpriteRender) {
        if block.kind != -1 {
            sprite.sprite_number = block.kind as usize * 9;
        }
        else {
            sprite.sprite_number = 8;
        }
    }
}
