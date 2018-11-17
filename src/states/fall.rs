use basics::block::Block;
use states::state::State;

// Fall gets called by Idle and makes the current Block fall downwards
// can also be interupted or switch states to Hang
pub struct Fall;

impl State for Fall {
    fn enter(b: &mut Block) {}

    fn execute(b: &mut Block) {
        println!("{}, fall also works", b.pos.0);
        b.kind = 1;
    }

    fn exit(b: &mut Block) {}
    fn counter_end(b: &mut Block) {}
}


