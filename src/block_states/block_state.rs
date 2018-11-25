use amethyst::ecs::prelude::{WriteStorage, Entity};
use basics::block::Block;
use block_states::{
    hang::Hang,
    land::Land,
    clear::Clear,
};

// A trait that all Block states should expand on
pub trait BlockState {
    // happens if loaded into another state
    fn enter(b: &mut Block);

    // happens when leaving to another state
    fn exit(b: &mut Block);

    // happens each frame,
    // takes an iterator - to know which block youre looking at right now
    // takes a reference of a vector of entities - access other components 
    // takes the whole stack of blocks - get ref or mut out of this
    fn execute(usize, &Vec<Entity>, &mut WriteStorage<'_, Block>);

    // gets called once the blocks counter runs down to 0
    // mostly used to switch states
    fn counter_end(usize, &Vec<Entity>, &mut WriteStorage<'_, Block>);
}

// changes the current blocks state to a new one
pub fn change_state(b: &mut Block, new_state: &'static str) {
    if b.state == new_state {
        return;
    }

    // call the currents state exit function
    match b.state {
        "LAND" => Land::exit(b),
        "CLEAR" => Clear::exit(b),
        _ => ()
    }  
        
    b.state = new_state;

    // call the currents state enter function
    match b.state {
        "HANG" => Hang::enter(b),
        "LAND" => Land::enter(b),
        "CLEAR" => Clear::enter(b),
        _ => ()
    }
}