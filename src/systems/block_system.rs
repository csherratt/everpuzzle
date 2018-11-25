use amethyst::{
    ecs::*,
    renderer::*,
    core::Transform,
};

use basics::{
    block::Block,
    stack::Stack,
};
use data::block_data::{COLS, BLOCKS};
use block_states::{
    block_state::BlockState,
    idle::Idle,
    hang::Hang,
    fall::Fall,
    land::Land,
    clear::Clear,
    swap::Swap,
};

// handles everything a block should do itself or based on others
pub struct BlockSystem;
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
                "CLEAR" => Clear::execute(i, &stack.entities, &mut blocks),
                "SWAP" => Swap::execute(i, &stack.entities, &mut blocks),
                _ => ()
            }

            // if the counter is at 0, call current states counter end function
            if blocks.get(stack.entities[i]).unwrap().counter <= 0 {
                match blocks.get(stack.entities[i]).unwrap().state {
                    "HANG" => Hang::counter_end(i, &stack.entities, &mut blocks),
                    "FALL" => Fall::counter_end(i, &stack.entities, &mut blocks),
                    "LAND" => Land::counter_end(i, &stack.entities, &mut blocks),
                    "CLEAR" => Clear::counter_end(i, &stack.entities, &mut blocks),
                    "SWAP" => Swap::counter_end(i, &stack.entities, &mut blocks),
                    _ => ()
                }    
            }
        }

        // translation
        for (b, transform) in (&blocks, &mut transforms).join() {
            transform.translation.x = b.x as f32 * transform.scale.x * 16.0 + b.offset.0;
            transform.translation.y = b.y as f32 * transform.scale.y * 16.0 + b.offset.1;
        }

        // rendering
        for (b, sprite) in (&mut blocks, &mut sprites).join() {
            update_sprites(b, sprite);
        }
    }
}

// visibility is on when the blocks kind isnt -1
// also sets the frame of the sprite by its kind * 9 and an additional 
// animation offset used to stay at specific horizontal sprites
fn update_sprites(b: &mut Block, sprite: &mut SpriteRender) {
    // decrease all the time
    if b.anim_counter > 0 {
        b.anim_counter -= 1;
    }

    // render sprite with kind when its not -1
    if b.kind != -1 && !b.clearing {
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

// checks wether the block below is empty or falling, also checks wether this block is empty
pub fn check_for_hang(i: usize, entities: &Vec<Entity>, blocks: &mut WriteStorage<'_, Block>) -> bool {
    // condition based on another block in a different lifetime
    let mut down_condition: bool = false;

    // check if is in vec boundary
    if i > COLS {
        let down = blocks.get_mut(entities[i - COLS]).unwrap();
        down_condition = down.is_empty() || down.state == "HANG";
    }

    !blocks.get_mut(entities[i]).unwrap().is_empty() && down_condition
}