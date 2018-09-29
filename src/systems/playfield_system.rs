use amethyst::ecs::*;
use basics::playfield::Playfield;
use amethyst::input::InputHandler;

pub struct PlayfieldSystem {
    pub is_down : bool
}

impl PlayfieldSystem {
    pub fn new() -> PlayfieldSystem {
        PlayfieldSystem {
            is_down: false,
        }
    }
}

impl<'a> System<'a> for PlayfieldSystem {
    type SystemData = (
        WriteStorage<'a, Playfield>,
        Read<'a, InputHandler<String, String>>
    );

    fn run(&mut self, (mut playfields, input): Self::SystemData) {
        // holding a button down or not
        if input.action_is_down("space").unwrap() {
            if !self.is_down {
                self.is_down = true;
                println!("DAYUM");

                for playfield in playfields.join() {
                    for b in &playfield.stack {

                        println!("{:?}", b);
                    }
                }
            }
        }
        else {
            self.is_down = false;
        }
    }
}
