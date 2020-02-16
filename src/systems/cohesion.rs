use amethyst::{
    core::SystemDesc,
    core::math::Vector2,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::boid::{Boid, Acceleration};

#[derive(SystemDesc)]
pub struct CohesionSystem;

impl<'s> System<'s> for CohesionSystem {
    type SystemData = (
        ReadStorage<'s, Boid>,
        WriteStorage<'s, Acceleration>,
    );

    fn run(&mut self, (boids, mut accelerations): Self::SystemData) {
        for (boid, acceleration) in (&boids, &mut accelerations).join() {
//            TODO implement cohesion
            acceleration.acceleration += Vector2::new(0.0, -2.0);
        }
    }
}