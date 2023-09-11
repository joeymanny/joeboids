use std::ops::Add;
pub trait BoidCanvas {
    fn draw_triangle(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Result<(), String>;
}
struct Boid<'a,T: BoidCanvas> {
    canvas: &'a mut T,
    b0: Vec<i32>,
    b1: Vec<i32>,
    switch: bool,
}
impl<U: BoidCanvas> Boid<'_, U> {
    fn new<T: BoidCanvas> (canvas: &mut T) -> Boid<T> {
        Boid {
            canvas: canvas,
            b0: Vec::<i32>::new(),
            b1: Vec::<i32>::new(),
            switch: false,
        }
    }
}
struct Boidee {
    pos: Vector2,
    dir: Angle,
    speed: f32,
}
struct Angle {
    rad: f32
}
impl Angle {
    fn new (x: f32) -> Angle {
        Angle {
            rad: x
        }
    }
}
struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
