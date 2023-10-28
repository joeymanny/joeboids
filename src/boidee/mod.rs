const PROTECTED_RANGE: f32 = 10.0;
const VISUAL_RANGE: f32 = 40.0;
const AVOID_FACTOR: f32 = 0.05;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FORCE: f32 = 0.0005;
const MAX_SPEED: f32 = 8.0;
const MIN_SPEED:f32 = 3.0;
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
        // rule 1
        new_boid.velocity += close * AVOID_FACTOR;

        if num_neighbors > 0 { // rules 2 and 3 divide by num_neighbors

            // rule 2
            velocity_avg = velocity_avg / num_neighbors as f32;
            new_boid.velocity += (velocity_avg - new_boid.velocity) * MATCHING_FACTOR;

            // rule 3
            position_avg = position_avg / num_neighbors as f32;
            new_boid.velocity += (position_avg - new_boid.position) * CENTERING_FORCE;
        }
        // temporary rule: try to get to the center
        new_boid.velocity += (Vector2{x: bounds.0 as f32 / 2.0, y: bounds.1 as f32 / 2.0} - new_boid.position) * 0.0005;
        // step forward
        new_boid.position += new_boid.velocity;

        // do donut world checks --------------------
        new_boid.position = Vector2{x:new_boid.position.x % bounds.0 as f32, y:new_boid.position.y % bounds.1 as f32};
        if new_boid.position.x < 0.0{
            new_boid.position.x += bounds.0 as f32;
        }
        if new_boid.position.y < 0.0{
            new_boid.position.y += bounds.1 as f32;
        } // end donut world checks ----------------

        // Once the velocity has been updated, compute the boid speed
        // speed = sqrt(boid.vx*boid.vx + boid.vy*boid.vy)
        // If speed>maxspeed:
        // boid.vx = (boid.vx/speed)*maxspeed
        // boid.vy = (boid.vy/speed)*minspeed
        // If speed<minspeed:
        // boid.vx = (boid.vx/speed)*minspeed
        // boid.vy = (boid.vy/speed)*minspeed
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