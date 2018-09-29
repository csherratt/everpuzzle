use amethyst::ecs::*;
use amethyst::renderer::*;
use basics::block::Block;

pub struct BlockSystem;

impl<'a> System<'a> for BlockSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        ReadStorage<'a,  Block>,
    );

    fn run(&mut self, (mut sprites, blocks): Self::SystemData) {
        for (sprite, block) in (&mut sprites, &blocks).join() {
            sprite.sprite_number = block.kind as usize * 9;
        }
    }
}
