pub mod block_data {
    pub const ROWS: usize = 12;
    pub const COLS: usize = 6;
    pub const BLOCKS: usize = ROWS * COLS;
}

pub mod helpers {
    use data::block_data::COLS;

    pub fn i2tuple(iterator: usize) -> (f32, f32) {
        let x = (iterator % COLS) as f32;
        let y = (iterator / COLS) as f32;

        (x, y.floor())
    }

    pub fn tuple2i(tuple: (f32, f32)) -> usize {
        tuple.1 as usize * COLS + tuple.0 as usize
    }
}
