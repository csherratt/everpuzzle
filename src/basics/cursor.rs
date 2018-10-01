use amethyst::ecs::prelude::{
    Component, DenseVecStorage
};

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub pos: (f32, f32),
    pub anim_offset: f32
}

impl Cursor {
    pub fn new(pos: (f32, f32)) -> Cursor {
        Cursor {
            pos,
            anim_offset: 0.0
        }
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}