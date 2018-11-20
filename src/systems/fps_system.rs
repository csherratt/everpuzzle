use amethyst::{
    utils::fps_counter::FPSCounter,
    core::timing::{duration_to_nanos, Time},
    ecs::*,
};

pub struct FPSSystem;

impl<'a> System<'a> for FPSSystem {
    type SystemData = (
        Read<'a, Time>,
        Write<'a, FPSCounter>,
    );

    fn run(&mut self, (time, mut counter): Self::SystemData) {
        counter.push(duration_to_nanos(time.delta_real_time())); 

        println!("fps: {}, sampled: {}", counter.frame_fps(), counter.sampled_fps());
    }
}
