#[cfg(test)]
mod tests;

mod grid;
mod angle;
pub mod boid;
mod vector2;
mod boidee;
pub const TOO_CLOSE: f32 = 15.0;
pub const LOCAL_SIZE: f32 = 50.0;
pub const MAX_RAND_SCOPE: usize = 100;
pub const  SCHEDULE_NANOS: u64 = 16_666_666; // 120 fps
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    ) -> Result<(), String>;
}