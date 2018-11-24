use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::{
    block::Block,
    stack::Stack,
};
use data::block_data::BLOCKS;
use block_states::{
    idle::Idle,
    hang::Hang,
    fall::Fall,
    land::Land,
    block_state::BlockState,
};

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
        Read<'a, Stack>,
    );

    fn run(&mut self, (
            mut sprites, 
            mut transforms, 
            mut blocks,
            stack,
            ): Self::SystemData)
    {
        // run through all states from a block
        for i in 0..BLOCKS {
            // decrease the counter if its over 0
            {
                let mut b = blocks.get_mut(stack.entities[i]).unwrap();
                
                if b.counter > 0 {
                    b.counter -= 1;
                }
            } 

            // match all on the blocks state - run all execute functions
            match blocks.get(stack.entities[i]).unwrap().state {
                "IDLE" => Idle::execute(i, &stack.entities, &mut blocks),
                "FALL" => Fall::execute(i, &stack.entities, &mut blocks),
                "LAND" => Land::execute(i, &stack.entities, &mut blocks),
                _ => ()
            }

            // if the counter is at 0, call current states counter end function
            if blocks.get(stack.entities[i]).unwrap().counter <= 0 {
                match blocks.get(stack.entities[i]).unwrap().state {
                    "HANG" => Hang::counter_end(i, &stack.entities, &mut blocks),
                    "FALL" => Fall::counter_end(i, &stack.entities, &mut blocks),
                    "LAND" => Land::counter_end(i, &stack.entities, &mut blocks),
                    _ => ()
                }    
            }
        }

        // translation
        for (block, transform) in (&blocks, &mut transforms).join() {
            transform.translation.x = block.x as f32 * transform.scale.x * 16.0;
            transform.translation.y = block.y as f32 * transform.scale.y * 16.0;
        }

        // rendering
        for (block, sprite) in (&mut blocks, &mut sprites).join() {
            BlockSystem::update_sprites(block, sprite);
        }
    }
}

impl BlockSystem {
    // visibility is on when the blocks kind isnt -1
    // also sets the frame of the sprite by its kind * 9 and an additional 
    // animation offset used to stay at specific horizontal sprites
    fn update_sprites(b: &mut Block, sprite: &mut SpriteRender) {
        // decrease all the time
        if b.anim_counter > 0 {
            b.anim_counter -= 1;
            println!("{}, should_animate", b.anim_counter);
        }

        if b.kind != -1 {
            if b.y == 0 {
                b.anim_offset = 1;
            }

            sprite.sprite_number = b.kind as usize * 9 + b.anim_offset as usize;
        }
        else {
            // static 0 alpha sprite rectangle
            sprite.sprite_number = 8;
        }
    }
}
