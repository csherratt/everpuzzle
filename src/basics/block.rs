use amethyst::ecs::prelude::{
    Component, DenseVecStorage
};

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub kind: i32
}

impl Block {
    pub fn new(new_kind: i32) -> Block {
        Block {
            kind: new_kind
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}