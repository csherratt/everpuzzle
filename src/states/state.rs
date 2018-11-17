use basics::block::Block; 

// Everything a normal Block State should supply.
//
// execute() happens every frame and is always called
// enter() happens once when switched to the specific State
// exit() happens once when being switched to another State
//
// counter_end() is special, its checked all the time and is more
// experimental. basically it just looks for the blocks counter and
// gets called once when the counter reached its limit - often used for
// switching states
pub trait State {
    fn execute(&mut Block);
    fn enter(&mut Block);
    fn exit(&mut Block);
    fn counter_end(&mut Block);
}
