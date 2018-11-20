use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Playfield {
    level: i32
}

impl Default for Playfield {
    fn default() -> Playfield {
        Playfield {
            level: 0 
        }
    }
}

impl Playfield {
    pub fn new() -> Playfield {
        Playfield { 
            ..Default::default()
        }
    }
}

impl Component for Playfield {
    type Storage = DenseVecStorage<Self>;
}
