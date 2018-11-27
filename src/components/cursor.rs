use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage},
};

pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub anim_offset: f32,
    pub offset: (f32, f32),
}

impl Default for Cursor {
    fn default() -> Cursor {
        Cursor {
            x: 0.0,
            y: 0.0,
            anim_offset: 0.0,
            offset: (0.0, 0.0),
        }
    }
}

impl Cursor {
    pub fn new(x: f32, y: f32) -> Cursor {
        Cursor {
            x,
            y,
            ..Default::default()
        }
    }

    pub fn set_position(&self, transform: &mut Transform) {
        transform.translation.x = (self.x * 32.0 + self.offset.0) * transform.scale.x;
        transform.translation.y = (self.y * 32.0 + self.offset.1) * transform.scale.y;
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}
