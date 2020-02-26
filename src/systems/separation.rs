use amethyst::{
    core::SystemDesc,
    core::math::{Vector3, Vector2},
    derive::SystemDesc,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::{Boid, Acceleration, Velocity};

pub const SEPARATION_RADIUS: f32 = 20.0;
pub const MAX_SPEED: f32 = 500.0;

#[derive(SystemDesc)]
pub struct SeparationSystem;

impl<'s> System<'s> for SeparationSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
    );

    fn run(&mut self, (boids, locals, velocities, mut accelerations): Self::SystemData) {
        for (_boid, local, velocity, acceleration) in (&boids, &locals, &velocities, &mut accelerations).join() {
            let desired: Vector3<f32> = (&locals, &boids).join()
                .filter_map(|(pos, _boid)| if pos.translation() != local.translation() && (pos.translation() - local.translation()).norm() <= SEPARATION_RADIUS {
                    // Some(*((pos.translation() - local.translation()).normalize() / ((pos.translation() - local.translation()).norm())))
                    Some((*local.translation() - *pos.translation()).normalize() / (pos.translation() - local.translation()).norm())
                } else{
                    None
                }).sum();

            if desired.x != 0.0 || desired.y != 0.0 {
                let difference = Vector2::new(desired.x, desired.y).normalize() * MAX_SPEED - velocity.velocity;
                acceleration.acceleration += difference;
            }
        }
    }
}