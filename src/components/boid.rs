use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    core::math::Vector3,
};
use std::default::Default;

pub struct Boid {}

impl Default for Boid{
    fn default() -> Self {
        Boid{}
    }
}

impl Component for Boid {
    type Storage = NullStorage<Self>;
}

pub struct Velocity {
    pub velocity: Vector3<f32>,
}

impl Velocity {
    pub fn new(velocity: Vector3<f32>) -> Velocity {
        Velocity {
            velocity
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct Acceleration {
    pub acceleration: Vector3<f32>,
}

impl Acceleration {
    pub fn new() -> Acceleration {
        Acceleration {
            acceleration: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Component for Acceleration {
    type Storage = DenseVecStorage<Self>;
}