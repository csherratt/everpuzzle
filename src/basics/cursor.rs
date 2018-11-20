use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};

pub struct Cursor {
    pub pos: (f32, f32),
    pub anim_offset: f32,
    pub offset: (f32, f32),
}

impl Default for Cursor {
    fn default() -> Cursor {
        Cursor {
            pos: (0.0, 0.0),
            anim_offset: 0.0,
            offset: (0.0, 0.0),
        }
    }
}

impl Cursor {
    pub fn new(pos: (f32, f32)) -> Cursor {
        Cursor { pos, ..Default::default() }
    }

    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 32.0 * transform.scale.x + self.offset.0;
        transform.translation.y = self.pos.1 * 32.0 * transform.scale.y + self.offset.1;
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}
