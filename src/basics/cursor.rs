use amethyst::ecs::prelude::{
    Component, DenseVecStorage
};

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub pos: (i32, i32),
}

impl Cursor {
    pub fn new(pos: (i32, i32)) -> Cursor {
        Cursor { pos }
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}