use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}, 
    core::Transform,
};

// Each direction a neighbor would be from the current id.
// so from 0 the right neighbor would be just one apart from it
// inside of an array that is
pub enum BlockNeighbors {
    DOWN = -6,
    TOP = 6,
    RIGHT = 1,
    LEFT = -1,
}

pub struct Block {
    pub kind: Option<i32>, // sprite_number or none
    pub pos: (f32, f32),
    pub can_fall: bool,
}

impl Block {
    pub fn new(kind: Option<i32>, pos: (f32, f32)) -> Block {
        Block {
            kind,
            pos,
            can_fall: false,
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
