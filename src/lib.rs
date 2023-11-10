mod grid;
pub mod boid;
mod vector2;
pub mod boidee;
pub const LOCAL_SIZE: f32 = 50.0;
pub use boid::TargetType;

/// Trait required for the [`Boid`](crate::boid::Boid) object to know how to draw [`Boidee`](crate::boidee::Boidee)s.
/// An object implementing this trait must be passed to your [`crate::boid::Boid`] when calling `step_draw` and
/// related functions
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    ) -> Result<(), String>;
}