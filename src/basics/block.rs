use amethyst::ecs::prelude::{
    Component, DenseVecStorage
};

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: i32,
    pub kind: Option<i32>, // sprite_number or none
    pub x: f32, 
    pub y: f32
}

impl Block {
    pub fn new(id: i32, kind: Option<i32>, x: f32, y: f32) -> Block {
        Block {
            id,
            kind,
            x,
            y
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
