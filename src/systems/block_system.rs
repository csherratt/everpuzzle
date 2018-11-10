use amethyst::ecs::*;
use amethyst::renderer::*;
use amethyst::input::*;
use rand::prelude::*;
use amethyst::core::*;
use amethyst::core::cgmath::Vector3;
use std::collections::HashMap;

use basics::block::Block;
use basics::cursor::Cursor;
use basics::rng_resource::RngResource;

pub struct BlockSystem {
    key_presses: HashMap<String, bool>
}

impl BlockSystem {
    pub fn new() -> BlockSystem {
        let mut key_presses: HashMap<String, bool> = HashMap::new();
        key_presses.insert(String::from("space"), false);
        key_presses.insert(String::from("swap"), false);

        BlockSystem {
            key_presses 
        }
    }

    pub fn press(&mut self, input: &mut Read<InputHandler<String, String>>, name: &str) -> bool {
        if input.action_is_down(name).unwrap() {
            if !*self.key_presses.get(name).unwrap() {
                *self.key_presses.get_mut(name).unwrap() = true;
                return true;
            }
        }
        else {
            *self.key_presses.get_mut(name).unwrap() = false;
        }

        return false;
    }
}

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Block>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<String, String>>,
        Write<'a, RngResource>,
        ReadStorage<'a, Cursor>
    );

    fn run(&mut self, (mut sprites, mut blocks, mut transform, mut input, mut generator, cursors): Self::SystemData) {
        if self.press(&mut input, "space") {
            for block in (&mut blocks).join() {
                let num = generator.rng.gen_range(0, 7);

                if num == 6 {
                    block.kind = None;
                }
                else {
                    block.kind = Some(num);
                }
            }
        }

        /*
        if self.press(&mut input, "swap") {
            for cursor in (cursors).join() {
                let mut chosen_blocks: Vec<Block> = Vec::new();

                for block in (&mut blocks).join() {
                    if (block.x, block.y) == cursor.pos {
                        if kind == None {
                            kind = block.kind;
                        }
                    }
                    else if (block.x + 1, block.y) == cursor.pos {
                        if kind != None {
                            block.
                        }
                    }
                }
            }

            println!("{:?}", chosen_blocks);
        }*/

        for (sprite, block, trans) in (&mut sprites, &mut blocks, &mut transform).join() {
            trans.translation = Vector3::new(
                block.x * 16.0 * trans.scale.x,
                block.y * 16.0 * trans.scale.y,
                0.0
            );

            if let Some(num) = block.kind {
                sprite.sprite_number = num as usize * 9;
            } else {
                sprite.sprite_number = 8;
            }
        }
    }
}
