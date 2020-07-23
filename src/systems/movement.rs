use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector3,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use std::f32::consts::PI;

pub const MAX_SPEED: f32 = 150.0;

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
        for (_boid, acceleration, velocity, local) in (&boids, &mut accelerations, &mut velocities, &mut locals).join() {
            velocity.velocity += acceleration.acceleration * time.delta_seconds();
            if velocity.velocity.norm_squared() > MAX_SPEED*MAX_SPEED {
                velocity.velocity = velocity.velocity.normalize() * MAX_SPEED
            }

            local.prepend_translation_x(velocity.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(velocity.velocity[1] * time.delta_seconds());

            // Angle from X axis to vector
            let roll = velocity.velocity.y.atan2(velocity.velocity.x);

            // Update local translation.
            local.set_rotation_euler(0.0, 0.0, roll - PI/2.0);

            // Reset acceleration vector
            acceleration.acceleration = Vector3::zeros();
        }
    }
}