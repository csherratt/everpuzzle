use amethyst::{
    ecs::prelude::{Entity, Component, DenseVecStorage}, 
    core::Transform,
};

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: Option<i32>, // sprite_number or none
    pub pos: (f32, f32),
    pub can_fall: bool,
    pub neighbor: Option<Entity>
}

impl Block {
    pub fn new(id: u32, kind: Option<i32>, pos: (f32, f32)) -> Block {
        Block {
            id,
            kind,
            pos,
            can_fall: false,
            neighbor: None
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
