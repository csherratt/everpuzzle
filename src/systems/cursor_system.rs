use amethyst::ecs::*;
use amethyst::core::Transform;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::*;
use amethyst::input::*;

use basics::cursor::Cursor;
use basics::rng_resource::RngResource;
use data::block_data::*;
use std::collections::HashMap;

pub struct CursorSystem {
    key_presses: HashMap<String, i32>
}

impl CursorSystem {
    pub fn new() -> CursorSystem {
        let mut key_presses: HashMap<String, i32> = HashMap::new();
        key_presses.insert(String::from("up"), 0);
        key_presses.insert(String::from("down"), 0);
        key_presses.insert(String::from("right"), 0);
        key_presses.insert(String::from("left"), 0);

        CursorSystem {
            key_presses
        }
    }

    pub fn hold(&mut self, input: &mut Read<InputHandler<String, String>>, name: &str) -> bool {
        if input.action_is_down(name).unwrap() {
            let result = *self.key_presses.get(name).unwrap();

            if result == 0 || result > 5 {
                *self.key_presses.get_mut(name).unwrap() += 1;
                return true;
            }

            *self.key_presses.get_mut(name).unwrap() += 1;
        }
        else {
            *self.key_presses.get_mut(name).unwrap() = 0;
        }

        return false;
    }
}

impl<'a> System<'a> for CursorSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Cursor>,
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut sprites, mut transforms, mut cursors, mut input): Self::SystemData) {
        if self.hold(&mut input, "up") {
            for cursor in (&mut cursors).join() {
                if cursor.pos.1 < (ROWS - 1) as f32 {
                    cursor.pos.1 += 1.0;
                }
            }
        }

        if self.hold(&mut input, "down") {
            for cursor in (&mut cursors).join() {
                if cursor.pos.1 > 0.0 {
                    cursor.pos.1 -= 1.0;
                }
            }
        }

        if self.hold(&mut input, "left") {
            for cursor in (&mut cursors).join() {
                if cursor.pos.0 > 0.0 {
                    cursor.pos.0 -= 1.0;
                }
            }
        }

        if self.hold(&mut input, "right") {
            for cursor in (&mut cursors).join() {
                if cursor.pos.0 < (COLS - 2) as f32 {
                    cursor.pos.0 += 1.0;
                }
            }
        }

        for (sprite, transform, cursor) in (&mut sprites, &mut transforms, &mut cursors).join() {
            transform.translation = Vector3::new(
                cursor.pos.0 * 32.0,
                cursor.pos.1 * 32.0,
                0.0
            );
            sprite.sprite_number = cursor.anim_offset as usize;

            if cursor.anim_offset < 7.0 {
                cursor.anim_offset += 1.0 / 2.0;
            }
            else {
                cursor.anim_offset = 0.0;
            }
        }
    }
}
