use amethyst::{
    core::SystemDesc,
    core::math::{Vector3},
    derive::SystemDesc,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::{Boid, Acceleration, Velocity};

pub const ALIGNMENT_RADIUS: f32 = 50.0;
pub const MAX_SPEED: f32 = 130.0;

fn dist(pos1: Vector3<f32>, pos2: Vector3<f32>) -> f32 {
    ((pos1.x - pos2.x).powf(2.0) + (pos1.y-pos2.y).powf(2.0)).sqrt()
}

#[derive(SystemDesc)]
pub struct AlignmentSystem;

impl<'s> System<'s> for AlignmentSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Acceleration>,
    );

    fn run(&mut self, (boids, velocities, locals, mut accelerations): Self::SystemData) {
        for (_boid, velocity, local, acceleration) in (&boids, &velocities, &locals, &mut accelerations).join() {
            let velocities: Vec<Vector3<f32>> = (&locals, &velocities).join()
                .filter(|(pos, _vel)| dist(*pos.translation() ,*local.translation()) <= ALIGNMENT_RADIUS)
                .map(|(_pos, vel)| vel.velocity).collect();

            let desired: Vector3<f32> =  velocities.iter().sum::<Vector3<f32>>()
                .component_div(&Vector3::new(velocities.len() as f32, velocities.len() as f32, velocities.len() as f32)).normalize()*MAX_SPEED;

            let difference = desired - velocity.velocity;

            acceleration.acceleration += difference;
        }
    }
}