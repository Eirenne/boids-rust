use amethyst::{
    core::SystemDesc,
    core::math::{Vector2, Vector3},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::{Boid, Acceleration, Velocity};

#[derive(SystemDesc)]
pub struct CohesionSystem;

pub const COHESION_RADIUS: f32 = 100.0;
pub const MAX_SPEED: f32 = 500.0;

impl<'s> System<'s> for CohesionSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
    );

    fn run(&mut self, (boids, locals, velocities, mut accelerations): Self::SystemData) {
        for (_boid, local, velocity, acceleration) in (&boids, &locals, &velocities, &mut accelerations).join() {

            let positions: Vec<Vector3<f32>> = (&locals, &boids).join()
                .filter(|(pos, _boid)| (pos.translation() - local.translation()).norm() <= COHESION_RADIUS)
                .map(|(pos, _boid)| *pos.translation())
                .collect();

            if positions.len() > 1 {
                let desired: Vector3<f32> =  positions.iter().sum::<Vector3<f32>>()
                    .component_div(&Vector3::new(positions.len() as f32, positions.len() as f32, positions.len() as f32)) - local.translation();


                let force = Vector2::new(desired.x, desired.y).normalize() * MAX_SPEED - velocity.velocity;
                acceleration.acceleration += force;
            }


        }
    }
}