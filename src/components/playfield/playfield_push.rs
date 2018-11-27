use amethyst::ecs::{Component, DenseVecStorage};

pub struct PlayfieldPush {
    pub any_clears: bool,
    pub any_top_blocks: bool,
    pub smooth_raise: bool,
    pub offset_counter: f32,
    pub raised_blocked_counter: u32,
    pub signal_raise: bool,
}

impl Default for PlayfieldPush {
    fn default() -> PlayfieldPush {
        PlayfieldPush {
            any_clears: false,
            any_top_blocks: false,
            smooth_raise: false,
            offset_counter: 0.0,
            raised_blocked_counter: 0,
            signal_raise: false,
        }
    }
}

impl Component for PlayfieldPush {
    type Storage = DenseVecStorage<Self>;
}
