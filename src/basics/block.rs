use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}, 
    core::Transform,
};

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: i32, // sprite_number or -1
    pub pos: (f32, f32),
    pub can_fall: bool,
    pub should_clear: bool
}

impl Block {
    pub fn new(id: u32, kind: i32, pos: (f32, f32)) -> Block {
        Block {
            id,
            kind,
            pos,
            can_fall: false,
            should_clear: false
        }
    }

    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 16.0 * transform.scale.x;
        transform.translation.y = self.pos.1 * 16.0 * transform.scale.y;
    }

    pub fn is_comboable(&mut self) -> bool {
        if self.pos.1 == 0.0 {
            return false
        }

        if self.kind != -1 {
            return true
        }

        return false
    }

    // wether this block is comboable and also matches with another kind
    pub fn is_comboable_with(&mut self, other: &mut Block) -> bool {
        if self.is_comboable() {
            if other.kind != -1 && other.kind == self.kind {
                return true
            }
        }

        // check if kinds exist, then compare them
        return false
    }

    // returns an array of comboable blocks including this block
    // wether 2 blocks have the same kind as this block - change state to CLEAR
    // returns an empty array otherwhise
    pub fn check_similar_blocks(&mut self, b1: Option<&mut Block>, b2: Option<&mut Block>) {
        if self.is_comboable() {
            if let Some(block1) = b1 {
                if let Some(block2) = b2 {
                    if block1.is_comboable_with(self) && block2.is_comboable_with(self) {
                        self.should_clear = true;
                        block1.should_clear = true;
                        block2.should_clear = true;
                    }
                }
            }
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
