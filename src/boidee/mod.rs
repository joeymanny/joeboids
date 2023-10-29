const PROTECTED_RANGE: f32 = 10.0;
const VISUAL_RANGE: f32 = 20.0;
const AVOID_FACTOR: f32 = 0.05;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FORCE: f32 = 0.0005;
const MAX_SPEED: f32 = 8.0;
const MIN_SPEED:f32 = 2.0;
const CENTER_FORCE: f32 = 0.0005;
const TARGET_FORCE: f32 = 0.001;
const TARGET_AVOID_DISTANCE: f32 = 300.0;
use crate::vector2::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;
#[derive(Clone, Copy)]
pub enum TargetType{
    Avoid,
    Approach
}
#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pub position: Vector2,
    pub velocity: Vector2,
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
    pub fn random(min: (f32, f32), max: (f32, f32)) -> Boidee {
        let mut r = rand::thread_rng();
        let dir = (r.gen::<f32>() * 2.0 * PI).sin_cos();
        Boidee {
            position: Vector2::new(
                (r.gen::<f32>() * max.0 - min.0 as f32) + min.0,
                (r.gen::<f32>() * max.1 - min.1 as f32) + min.0,
            ),
            velocity: Vector2::new(dir.0, dir.1),
        }
    }
    #[allow(unused)]
    pub fn new() -> Boidee {
        Boidee {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(1.0, 0.0),
        }
    }
    pub fn step(
        &self,
        nearby_boids: Vec<Boidee>,
        min: (f32, f32),
        max: (f32, f32),
        flock_scare: Option<f32>,
        target: Option<(Vector2, TargetType)>
    ) -> Boidee
    {
        let mut new_boid = self.clone();
        let mut close: Vector2 = Vector2::zero();
        let mut velocity_avg = Vector2::zero();
        let mut position_avg = Vector2::zero();
        let mut num_neighbors: u32 = 0;
    // we're gonna return a modified version of ourself
        for near in nearby_boids{
            let distance = (near.position - self.position).abs();
            if distance < VISUAL_RANGE{
                num_neighbors += 1;
                velocity_avg += near.velocity;
                position_avg += near.position;
                if distance < PROTECTED_RANGE {
                    close += self.position - near.position;
                }
            }
        }
        let scare_factor = match flock_scare{
            Some(v) => v,
            None => 1.0
        };
        // rule 1 - seperation
        new_boid.velocity += close * AVOID_FACTOR;

        if num_neighbors > 0 { // rules 2 and 3 divide by num_neighbors - it can't be zero

            // rule 2 - alignment
            velocity_avg = velocity_avg / num_neighbors as f32;
            new_boid.velocity += (velocity_avg - new_boid.velocity) * MATCHING_FACTOR * scare_factor;

            // rule 3 - cohesion
            position_avg = position_avg / num_neighbors as f32;
            new_boid.velocity += (position_avg - new_boid.position) * CENTERING_FORCE * scare_factor;
        }

        // temporary rule: try to get to the center
        new_boid.velocity += (Vector2{x: ((max.0 - min.0) as f32 / 2.0) + min.0 as f32, y: ((max.1 - min.1) as f32 / 2.0) + min.1 as f32} - new_boid.position) * CENTER_FORCE;

        // targeting: avoid or approach any targets
        if let Some(config) = target{
            let target_type = if let TargetType::Avoid = config.1{
                let distance = (config.0 - self.position).abs();
                if distance < TARGET_AVOID_DISTANCE{
                    1.0 / distance * -TARGET_AVOID_DISTANCE
                } else {1.0}
            }else{
                1.0
            };
            new_boid.velocity += (config.0 - new_boid.position) * TARGET_FORCE * target_type;
        }
        // step forward
        new_boid.position += new_boid.velocity;

        // do donut world checks --------------------
        // end donut world checks ----------------

        let speed = new_boid.velocity.abs();
        if speed > MAX_SPEED{
            new_boid.velocity = new_boid.velocity / speed;
            new_boid.velocity = new_boid.velocity * MAX_SPEED;
        } else if speed < MIN_SPEED{
            new_boid.velocity = new_boid.velocity / speed;
            new_boid.velocity = new_boid.velocity * MIN_SPEED;
        }
        new_boid
        // TODO
    }
}