use amethyst::ecs::*;
use amethyst::renderer::*;
use amethyst::input::*;
use rand::prelude::*;

use basics::block::Block;
use basics::rng_resource::RngResource;

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
        Write<'a, RngResource>
    );

    fn run(&mut self, (mut sprites, mut blocks, input, mut generator): Self::SystemData) {
      if input.action_is_down("space").unwrap() {
            if self.space_pressed == 0 {
                for block in (&mut blocks).join() {
                    block.kind = generator.rng.gen_range(0, 6);
                }
            }

            self.space_pressed += 1;
        }
        else {
            self.space_pressed = 0;
        }

        if input.is_controller_connected(0) {
            println!("DAMN");
        }

        if input.controller_button_is_down(0, ControllerButton::A) {
            println!("INPUT IS WORKING");
        }

        for (sprite, block) in (&mut sprites, &mut blocks).join() {
            sprite.sprite_number = block.kind as usize * 9;
        }
    }
}
