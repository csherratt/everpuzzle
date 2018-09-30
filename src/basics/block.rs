use amethyst::ecs::prelude::{
    Component, DenseVecStorage
};

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub id: i32,
    pub kind: i32
}

impl Block {
    pub fn new(id: i32, kind: i32) -> Block {
        Block {
            id,
            kind
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}