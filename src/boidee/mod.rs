const PROTECTED_RANGE: f32 = 4.0;
const AVOID_FACTOR: f32 = 1.0;
use crate::vector2::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;
#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pub position: Vector2,
    pub velocity: Vector2,

    chosen: bool
}
impl std::fmt::Display for Boidee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: ({},{}), velocity: ({},{})",
            self.position.x, self.position.y, self.velocity.x, self.velocity.y
        )
    }
}
impl Boidee {
    pub fn random(bounds: (usize, usize)) -> Boidee {
        let mut r = rand::thread_rng();
        let dir = (r.gen::<f32>() * 2.0 * PI).sin_cos();
        Boidee {
            position: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            velocity: Vector2::new(dir.0, dir.1),
            chosen: false
        }
    }
    pub fn new() -> Boidee {
        Boidee {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(1.0, 0.0),
            chosen: false
        }
    }
    pub fn step(
        &self,
        nearby_boids: Vec<Boidee>,
        bounds: (usize, usize),
        flock_scare: Option<f32>
    ) -> Boidee
    {
        let mut new_boid = self.clone();
        let mut close: Vector2 = Vector2{x: 0.0, y: 0.0};
    // we're gonna return a modified version of ourself
        for near in nearby_boids{
            // if it's within 'sight'
            if (near.position - self.position).abs() < PROTECTED_RANGE {
                close += self.position - near.position;
            }
        }
        new_boid.velocity += close * AVOID_FACTOR;
        new_boid.position += new_boid.velocity;
        new_boid.position = Vector2{x:new_boid.position.x % bounds.0 as f32, y:new_boid.position.y % bounds.1 as f32};
        if new_boid.position.x < 0.0{
            new_boid.position.x += bounds.0 as f32;
        }
        if new_boid.position.y < 0.0{
            new_boid.position.y += bounds.1 as f32;
        }
        new_boid
        // TODO
    }
}