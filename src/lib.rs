#[cfg(test)]
mod tests;
mod grid;
mod angle;
mod boid;
mod vector2;
pub const SIZE_FACTOR: f32 = 8.0;
pub const TOO_CLOSE: f32 = 15.0;
pub const LOCAL_SIZE: f32 = 50.0;
pub const MAX_RAND_SCOPE: f32 = 3.0;
pub const  SCHEDULE_NANOS: u64 = 1_000_000;

use rand::Rng;
use std::f32::consts::PI;
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::time::Instant;
use std::ops::Deref;
use std::fmt::Display;
use std::ops::Rem;
use std::cmp::Ordering;
use std::time::Duration;
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    ) -> Result<(), String>;
}



// fn avoid_point(curr: f32, point: Vector2) -> f32 {
    // let point = point * -1.0;
    // face_point(curr, point)
// }
// fn face_point(curr: f32, point: Vector2) -> f32 {
//     let wish = atan2_to_total(point.y.atan2(point.x));
//     let mut means = wish - curr;
//     // check if theres a closer way going to opposite direction
//     if means > PI {
//         means = (-2.0 * PI) + means
//     }
//     if means < (-1.0 * PI) {
//         means = (2.0 * PI) + means
//     }
//     means % (2.0 * PI)
// }
// fn atan2_to_total(n: f32) -> f32 {
//     if n.is_sign_negative() {
//         (2.0 * PI) + n
//     } else {
//         n
//     }
// }
