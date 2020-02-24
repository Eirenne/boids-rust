use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::math::Vector2,
};
pub const BOID_HEIGHT: f32 = 16.0;
pub const BOID_WIDTH: f32 = 4.0;

pub struct Boid {
    pub width: f32,
    pub height: f32,
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            width: BOID_WIDTH,
            height: BOID_HEIGHT,
        }
    }
}

impl Component for Boid {
    type Storage = DenseVecStorage<Self>;
}

pub struct Velocity {
    pub velocity: Vector2<f32>,
}

impl Velocity {
    pub fn new(velocity: Vector2<f32>) -> Velocity {
        Velocity {
            velocity
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct Acceleration {
    pub acceleration: Vector2<f32>,
}

impl Acceleration {
    pub fn new() -> Acceleration {
        Acceleration {
            acceleration: Vector2::new(0.0, 0.0)
        }
    }
}

impl Component for Acceleration {
    type Storage = DenseVecStorage<Self>;
}