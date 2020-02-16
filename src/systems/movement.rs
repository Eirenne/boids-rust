use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector2,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::{Boid, Acceleration, Velocity};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (boids, mut accelerations, mut velocities, mut locals, time): Self::SystemData) {
        for (boid, acceleration, velocity, local) in (&boids, &mut accelerations, &mut velocities, &mut locals).join() {

            velocity.velocity += acceleration.acceleration * time.delta_seconds();
            local.prepend_translation_x(velocity.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(velocity.velocity[1] * time.delta_seconds());
            acceleration.acceleration = Vector2::zeros();
        }
    }
}