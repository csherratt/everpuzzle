use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub pos: (f32, f32),
    pub anim_offset: f32,
}

impl Cursor {
    pub fn new(pos: (f32, f32)) -> Cursor {
        Cursor {
            pos,
            anim_offset: 0.0,
        }
    }

    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 32.0 * transform.scale.x;
        transform.translation.y = self.pos.1 * 32.0 * transform.scale.y;
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}
