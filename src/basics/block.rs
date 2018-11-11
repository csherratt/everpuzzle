use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}, 
    core::Transform
};

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub kind: Option<i32>, // sprite_number or none
    pub pos: (f32, f32),
    pub can_fall: bool
}

impl Block {
    pub fn new(kind: Option<i32>, pos: (f32, f32)) -> Block {
        Block {
            kind,
            pos,
            can_fall: false
        }
    }

    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 16.0 * transform.scale.x;
        transform.translation.y = self.pos.1 * 16.0 * transform.scale.y;
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
