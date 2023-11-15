//! Crate for getting boid simulations anywhere a triangle can be drawn. This crate is CPU based, which isn't ideal, but this is a toy crate.
//! It's kinda fun. You can check out some (2) examples at the [repo](https://github.com/joeymanny/joeboids).



/// Main [`Boid`](`crate::boid::Boid`) struct, holds all [`Boidee`](`crate::boidee::Boidee`)s and logic
pub mod boid;
/// Type representing a single bird, just a position and velocity
pub mod boidee;

mod grid;
mod vector2;
pub mod prelude{
    pub use crate::TargetType;
    pub use crate::boid::Boid;
    pub use crate::boidee::Boidee;
    pub use crate::BoidCanvas;
    }
/// Whether to avoid or approach the target. Used for [step_on_schedule](crate::boid::Boid::step_on_schedule) and [raw_step](crate::boid::Boid::raw_step)
#[derive(Clone, Copy)]
pub enum TargetType{
    Avoid,
    Approach
}
/// Trait required for the [`Boid`](`crate::boid::Boid`) object to know how to draw [`Boidee`](`crate::boidee::Boidee`)s.
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