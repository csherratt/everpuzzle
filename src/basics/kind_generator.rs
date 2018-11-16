use rand::prelude::*;
use data::block_data::{
    COLS, 
    ROWS,
    BLOCKS,
};


// resource that stores the rng generator that will be global
// accessed via the world
#[derive(Debug)]
pub struct KindGenerator {
    pub rng: SmallRng,
}

impl Default for KindGenerator {
    // default so it can be fetched by systems
    fn default() -> KindGenerator {
        KindGenerator {
            rng: SmallRng::from_seed([0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        }
    }
}

// returns a stack of blocks where no nzmbers are the same next to each other
// also nulls kinds randomly and creates holes throughout the stack
// also has zones in which all blocks will definitely be nulled
// and a safe zone where no nulling happens
impl KindGenerator {
    pub fn create_stack(&mut self, safe: usize, nulling: usize) -> Vec<i32> {
        let safe_zone: usize = safe * COLS;
        let nulling_zone: usize = nulling * COLS;

        // empty array to destined length
        let size: usize = BLOCKS; //TODO: ROWS_VIS data
        let mut nums: Vec<i32> = Vec::new();
        nums.resize(size, -1);

        // scoped previous number that saves the newest generated number 
        let mut prev_before = -1;

        for i in 0..size {
            let mut new_num: i32 = 0;
            let mut bot_num: i32 = -1; // by default -1
            let mut skip: bool = false;

            // set bot_num once it respects the boundaries
            if i > COLS {
                bot_num = nums[i - COLS];

                // if bot_num is -1, just set new_num to -1 and skip
                if bot_num == -1 {
                    skip = true;
                    new_num = -1;
                }
            }

            if !skip {
                // when over start to go through
                if i != 0 {
                    // if the right wall is hit (after i * 6) then be true
                    if i % COLS + 1 != 0 {
                        new_num = self.get_number_in_zone(prev_before, bot_num, i, safe_zone, nulling_zone);
                    }
                    else {
                        new_num = self.get_number_in_zone(-1, bot_num, i, safe_zone, nulling_zone);
                    }
                }
                else {
                    new_num = self.get_number_in_zone(-1, -1, i, safe_zone, nulling_zone);
                }
            }

            prev_before = new_num; 
            nums[i] = new_num;
        }
        
        nums
    }

    // returns a randomly chosen number out of an array
    // you can erase contents inside by specifying them in the parameters
    // otherwhise theyll remain available to the chosen randomly
    fn get_number_in_zone(
        &mut self,
        cond1: i32, 
        cond2: i32, 
        iterator: usize, 
        safe_zone: usize, 
        null_zone: usize
    ) -> i32 {
        let mut numbers: Vec<i32> = vec![-1, 0, 1, 2, 3, 4];
        
        if iterator >= null_zone {
            return -1; 
        }

        if safe_zone >= iterator {
            numbers.retain(|x| x != &-1); // leave everything but -1
        }

        if cond1 != -1 {
            numbers.retain(|x| x != &cond1);
        }

        if cond2 != -1 {
            numbers.retain(|x| x != &cond2);
        }

        return numbers[self.rng.gen_range(0, numbers.len())];
    }
}
