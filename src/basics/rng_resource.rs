use rand::prelude::*;

// resource that stores the rng generator that will be global
// accessed via the world
#[derive(Debug)]
pub struct RngResource {
    pub rng: SmallRng
}

impl Default for RngResource {
    // default so it can be fetched by systems
    fn default() -> RngResource {
        RngResource {
            rng: SmallRng::from_seed([0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        }
    }
}
