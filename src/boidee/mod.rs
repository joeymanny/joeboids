const RAND_BOID_SPEED: f32 = 4.0;
const RAND_BOID_SPEED_VARIATION: f32 = 0.5;
use crate::vector2::Vector2;
use crate::angle::Angle;
use crate::{MAX_RAND_SCOPE, TOO_CLOSE, LOCAL_SIZE};
use rand::prelude::*;
use std::f32::consts::PI;
use crate::grid::Grid;
#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pub pos: Vector2,
    pub velocity: Vector2,

    chosen: bool
}
impl std::fmt::Display for Boidee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: ({},{}), velocity: ({},{})",
            self.pos.x, self.pos.y, self.velocity.x, self.velocity.y
        )
    }
}
impl Boidee {
    pub fn random(bounds: (usize, usize)) -> Boidee {
        let mut r = rand::thread_rng();
        let dir = (r.gen::<f32>() * 2.0 * PI).sin_cos();
        Boidee {
            pos: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            velocity: Vector2::new(dir.0, dir.1),
            chosen: false
        }
    }
    pub fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(1.0, 0.0),
            chosen: false
        }
    }
    pub fn step(
        &self,
        flock: &Grid,
        bounds: (usize, usize),
        flock_scare: Option<f32>
    ) -> Boidee
    {
        self.clone()
    }
}