

// const PROTECTED_RANGE: f32 = 8.0;
// const VISUAL_RANGE: f32 = 40.0;
const AVOID_FACTOR: f32 = 0.05;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FORCE: f32 = 0.0005;
const MAX_SPEED: f32 = 7.0;
const MIN_SPEED:f32 = 3.0;
const TARGET_FORCE: f32 = 0.001;
const EDGE_AVOIDANCE_FORCE: f32 = 0.07;
const EDGE_AVOIDANCE_MARGIN: f32 = 6.0; // boids will avoid the edge when one tenth of the whole screen from the edge
const TARGETING_VISUAL_BOOST_FAC: f32 = 5.0;
const FRICTION: f32 = 0.999;

use crate::vector2::Vector2;
use rand::prelude::*;
use std::f32::consts::PI;


/// The birds pushed around. These are simply a velocity and position. They have a min and max speed and a tiny drag force that 
/// decays their velocity every step. Their vision is 5x further when it comes to [`target`](crate::TargetType)s.
#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pub position: Vector2,
    pub velocity: Vector2,
    #[cfg(feature = "visualize_neighbors")]
    pub chosen: bool
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
    /// Returns a Self with a random normalized direction and a random position between `min` and `max`.
    pub fn random(min: (f32, f32), max: (f32, f32)) -> Boidee {
        let mut r = rand::thread_rng();
        let dir = (r.gen::<f32>() * 2.0 * PI).sin_cos();
        Boidee {
            position: Vector2::new(
                (r.gen::<f32>() * max.0 - min.0 as f32) + min.0,
                (r.gen::<f32>() * max.1 - min.1 as f32) + min.0,
            ),
            velocity: Vector2::new(dir.0, dir.1),
            #[cfg(feature = "visualize_neighbors")]
            chosen: false
        }
    }
    /// Returns a new Self with position 0,0 and velocity 1,0.
    #[allow(unused)]
    pub fn new() -> Boidee {
        Boidee {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(1.0, 0.0),
            #[cfg(feature = "visualize_neighbors")]
            chosen: false
        }
    }
    /// Returns a stepped Self based off the parameters passed in.
    pub fn step(
        &self,
        nearby_boids: Vec<&Boidee>,
        min: (f32, f32),
        max: (f32, f32),
        flock_scare: Option<f32>,
        target: Option<((f32, f32), crate::TargetType)>,
        visual_range: f32,
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
            if distance < visual_range{
                num_neighbors += 1;
                velocity_avg += near.velocity;
                position_avg += near.position;
                if distance < (visual_range * 0.2 ) { // protected range is 1/40th visual range (just the ratio i used before geting rid of consts)
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


        // targeting: avoid or approach any targets
        if let Some(config) = target{
            let target_pos = Vector2::new(config.0.0, config.0.1);
            let distance = (target_pos - self.position).abs();
            if distance < visual_range * TARGETING_VISUAL_BOOST_FAC{
            let target_fac = if let crate::TargetType::Avoid = config.1{
                    1.0 / distance * -visual_range * TARGETING_VISUAL_BOOST_FAC
            }else{
                1.0 / distance * visual_range * TARGETING_VISUAL_BOOST_FAC
            };
            new_boid.velocity += (target_pos - new_boid.position) * TARGET_FORCE * target_fac;
        }
        }
        let x_border = (max.0 - min.0) / EDGE_AVOIDANCE_MARGIN;
        let y_border = (max.1 - min.1) / EDGE_AVOIDANCE_MARGIN;
        if new_boid.position.x > max.0 - x_border{
            new_boid.velocity += Vector2::left() * EDGE_AVOIDANCE_FORCE;
        }
        if new_boid.position.y > max.1 - y_border{
            new_boid.velocity += Vector2::down() * EDGE_AVOIDANCE_FORCE;
        }
        if new_boid.position.x < min.0 + x_border{
            new_boid.velocity += Vector2::right() * EDGE_AVOIDANCE_FORCE;
        }
        if new_boid.position.y < min.1 + y_border{
            new_boid.velocity += Vector2::up() * EDGE_AVOIDANCE_FORCE;
        }
        // step forward
        new_boid.position += new_boid.velocity;


        new_boid.velocity = new_boid.velocity *  FRICTION;
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