mod grid;
pub mod boid;
mod vector2;
pub mod boidee;
pub use boid::TargetType;
pub use boid::Boid;
pub use boidee::Boidee;

/// Trait required for the [`Boid`](crate::boid::Boid) object to know how to draw [`Boidee`](crate::boidee::Boidee)s.
/// An object implementing this trait must be passed to your [Boid](`crate::boid::Boid`) when calling `step_draw` and
/// related functions
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
    ) -> Result<(), String>;
}