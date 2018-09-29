use amethyst::prelude::*;
use basics::block::Block;
use data::block_data::BLOCKS;

use amethyst::core::{
    Transform, GlobalTransform,
};
use amethyst::core::cgmath::Vector3;

use amethyst::renderer::*;
use amethyst::ecs::*;
use data::helpers::i2tuple;
use basics::spritesheet_loader::load_sprite_sheet;
use rand::prelude::*;

#[derive(Debug)]
pub struct Playfield {
    pub stack: Vec<Entity>,
    pub rng: SmallRng
}

impl Component for Playfield {
    type Storage = DenseVecStorage<Self>;
}

impl Playfield {
    pub fn create(&mut self, world: &mut World) {
        world.register::<Block>();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            let (x, y): (f32, f32) = i2tuple(i);
            let scale_amount = Vector3::new(2.0, 2.0, 2.0);
            trans.translation = Vector3::new(
                (x * 16.0) * scale_amount.x,
                (y * 16.0) * scale_amount.y,
                0.0
            );
            trans.scale = scale_amount;

            let sprite_render_block = SpriteRender {
                sprite_sheet: load_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            self.stack.push(world
                .create_entity()
                .with(sprite_render_block)
                .with(Block::new(self.rng.gen_range(0, 6)))
                .with(GlobalTransform::default())
                .with(trans)
                .build());
        }
    }
}