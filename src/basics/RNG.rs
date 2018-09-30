use rand::prelude::*;

#[derive(Debug)]
pub struct RNG {
    pub gen: SmallRng
}

impl Default for RNG {
    fn default() -> RNG {
        RNG {
            gen: SmallRng::from_seed([0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        }
    }
}
