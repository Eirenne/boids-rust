use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
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