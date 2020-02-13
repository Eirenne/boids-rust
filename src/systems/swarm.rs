use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::Boid;

#[derive(SystemDesc)]
pub struct SwarmSystem;

impl<'s> System<'s> for SwarmSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (boids, mut locals, time): Self::SystemData) {
        for (boid, local) in (&boids, &mut locals).join() {
//            local.prepend_translation_x(boid.velocity[0] * time.delta_seconds());
//            local.prepend_translation_y(boid.velocity[1] * time.delta_seconds());
        }
    }
}