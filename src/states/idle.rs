use basics::block::Block;
use states::state::State;

// Idle will be the normal state where blocks check to transition to
// other states and start applying logic
pub struct Idle;

impl State for Idle {
    fn enter(b: &mut Block) {}

    fn execute(b: &mut Block) {
        println!("{}, shit mate this works", b.pos.0);
        b.kind = 0;
    }

    fn exit(b: &mut Block) {}
    fn counter_end(b: &mut Block) {}
}


