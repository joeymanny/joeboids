// TODO error enum
//
// NOTE: take BoidCanvas as arg to step_draw(T: BoidCanvas) function rather 
// than storing a reference to canvas inside Boid
use rand::Rng;
use std::iter::zip;
use std::fmt;
use std::ops::Add;
pub trait BoidCanvas {
    fn draw_triangle(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Result<(), String>;
}
pub struct Boid {
    b0: Vec<Boidee>,
    b1: Vec<Boidee>,
    switch: bool,
}
impl Boid {
    pub fn new() -> Boid {
        Boid {
            b0: Vec::<Boidee>::new(),
            b1: Vec::<Boidee>::new(),
            // indicates which buffer has the most up-to-date
            // data
            // false = b0,
            // true = b1
            switch: false,
        }
    }
    pub fn init_boidee(&mut self, num: u32) {
        for _ in 0..num {
            self.b0.push(Boidee::new());
            self.b1.push(Boidee::new());
        }
        // make sure we start knowing buffer 0 has the data
        self.switch = false;
    }
    pub fn step<T: BoidCanvas>(&mut self, canvas: &mut T) {
        // target buffer
        let mut b;
        // buffer containing most up-to-date boids
        let c;
        if self.switch {
            b = &mut self.b0;
            c = &self.b1;
        } else {
            b = &mut self.b1;
            c = &self.b0;
        }
        for (current, buffer) in zip(c, b) {
            let new_boid = current.step(c);
            println!("{}", &new_boid);
            canvas.draw_triangle(
                ((new_boid.pos.x - 5.0) as i32, (new_boid.pos.y + 5.0) as i32),
                ((new_boid.pos.x + 5.0) as i32, (new_boid.pos.y + 5.0) as i32),
                ((new_boid.pos.x) as i32, (new_boid.pos.y - 5.0) as i32)
            );
            *buffer = new_boid;
        }
        self.switch = !self.switch;
    }
}
struct Boidee {
    pos: Vector2,
    dir: f32,
    speed: f32,
}
impl fmt::Display for Boidee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos: ({},{}) bearing: {}° speed: {}", self.pos.x, self.pos.y, self.dir, self.speed)
    }
}

impl Boidee {
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: 0.0,
            speed: rand::thread_rng().gen::<f32>() * 5.0,
        }
    }
    fn step(&self, flock: &Vec<Boidee>) -> Boidee {
        let new_pos = self.pos + Vector2::new(self.dir.sin() * self.speed, self.dir.cos() * self.speed);
        Boidee {
            pos: new_pos,
            dir: self.dir.clone() + 0.01,
            speed: self.speed.clone(),
        }
    }
}
//struct Angle {
//    rad: f32
//}
//impl Angle {
//    fn new (x: f32) -> Angle {
//        Angle {
//            rad: x
//        }
//    }
//}
#[derive (Clone, Copy)]
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
