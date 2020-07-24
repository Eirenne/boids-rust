use amethyst::{
    core::SystemDesc,
    core::math::{Vector3, zero},
    derive::SystemDesc,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage, Entities},
};

use crate::components::boid::{Boid, Acceleration, Velocity, VectorExt};



pub const SEPARATION_RADIUS: f32 = 20.0;
pub const ALIGNMENT_RADIUS: f32 = 50.0;
pub const COHESION_RADIUS: f32 = 50.0;
pub const BORDER_RADIUS: f32 = 30.0;
pub const MAX_SPEED: f32 = 150.0;

#[derive(SystemDesc)]
pub struct BoidSystem;


impl<'s> System<'s> for BoidSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Boid>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
    );

    fn run(&mut self, (entities, boids, locals, velocities, mut accelerations): Self::SystemData) {
        for (entity, _boid, local, velocity, acceleration) in (&entities, &boids, &locals, &velocities, &mut accelerations).join() {
            let mut center_mass = zero::<Vector3<f32>>();
            let mut desired_separation = zero::<Vector3<f32>>();
            let mut average_velocity = zero::<Vector3<f32>>();
            let mut cohesion_count = 0;
            let mut separation_count = 0;
            let mut alignment_count = 0;
            for (other_local, _other_boid, other_entity, other_velocity) in (&locals, &boids, &entities, &velocities).join(){
                if other_entity != entity {
                    let distance = (other_local.translation() - local.translation()).magnitude();
                    if distance < COHESION_RADIUS {
                        center_mass += *other_local.translation();
                        cohesion_count += 1;
                    }

                    if distance < SEPARATION_RADIUS {
                        desired_separation += (local.translation() - other_local.translation()).normalize() / distance;
                        separation_count += 1;
                    }

                    if distance < ALIGNMENT_RADIUS{
                        average_velocity += other_velocity.velocity;
                        alignment_count += 1;
                    }

                }

            }
            let cohesion_force = if cohesion_count > 0 {
                center_mass /= cohesion_count as f32;
                (center_mass - local.translation()).normalize()*MAX_SPEED - velocity.velocity
            } else {
                zero::<Vector3<f32>>()
            };

            let alignment_force = if alignment_count > 0 {
                average_velocity /= alignment_count as f32;
                average_velocity.normalize()*MAX_SPEED - velocity.velocity
            } else {
                zero::<Vector3<f32>>()
            };

            let separation_force = if separation_count > 0 {
                desired_separation /= separation_count as f32;
                desired_separation.normalize()*MAX_SPEED - velocity.velocity

            } else {
                zero::<Vector3<f32>>()
            };


            let position = local.translation();

            let mut border_steer = zero::<Vector3<f32>>();


            // TODO: rewrite borders
            if position[0] < BORDER_RADIUS {
                border_steer += (position - Vector3::new(0.0, position[1], 0.0)).normalize() *
                    (3000.0 * (BORDER_RADIUS-(position - Vector3::new(0.0, position[1], 0.0)).norm())/BORDER_RADIUS)
            }

            if position[0] > 800.0 - BORDER_RADIUS {
                border_steer += (position - Vector3::new(800.0, position[1], 0.0)).normalize() *
                    (3000.0 * (BORDER_RADIUS-(position - Vector3::new(800.0, position[1], 0.0)).norm())/BORDER_RADIUS)
            }

            if position[1] < BORDER_RADIUS {
                border_steer += (position - Vector3::new(position[0], 0.0, 0.0)).normalize() *
                    (3000.0 * (BORDER_RADIUS-(position - Vector3::new(position[0], 0.0, 0.0)).norm())/BORDER_RADIUS)
            }

            if position[1] > 400.0 - BORDER_RADIUS {
                border_steer += (position - Vector3::new(position[0], 400.0, 0.0)).normalize() *
                    (3000.0 * (BORDER_RADIUS-(position - Vector3::new(position[0], 400.0, 0.0)).norm())/BORDER_RADIUS)
            }

            acceleration.acceleration += Vector3::new(border_steer[0], border_steer[1], 0.0);
            acceleration.acceleration += alignment_force.limit(MAX_SPEED) * 2.0;
            acceleration.acceleration += cohesion_force.limit(MAX_SPEED) * 1.0;
            acceleration.acceleration += separation_force.limit(MAX_SPEED) * 1.5;

        }
    }
}