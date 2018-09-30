use amethyst::ecs::*;
use amethyst::renderer::*;
use amethyst::input::InputHandler;
use rand::prelude::*;

use basics::block::Block;
use basics::RNG::RNG;

pub struct BlockSystem {
    space_pressed: i32
}

impl BlockSystem {
    pub fn new() -> BlockSystem {
        BlockSystem {
            space_pressed: 0
        }
    }
}

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Block>,
        Read<'a, InputHandler<String, String>>,
        Write<'a, RNG>
    );

    fn run(&mut self, (mut sprites, mut blocks, input, mut generator): Self::SystemData) {
      if input.action_is_down("space").unwrap() {
            if self.space_pressed == 0 {
                for block in (&mut blocks).join() {
                    block.kind = generator.gen.gen_range(0, 6);
                }
            }

            self.space_pressed += 1;
        }
        else {
            self.space_pressed = 0;
        }

        for (sprite, block) in (&mut sprites, &mut blocks).join() {
            sprite.sprite_number = block.kind as usize * 9;
        }
    }
}
