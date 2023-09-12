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
pub struct Boid{
    bounds: (u32, u32),
    b0: Vec<Boidee>,
    b1: Vec<Boidee>,
    switch: bool,
}
impl Boid {
    pub fn new(bounds: (u32, u32)) -> Boid {
        Boid {
            bounds: bounds,
            b0: Vec::<Boidee>::new(),
            b1: Vec::<Boidee>::new(),
            // indicates which buffer has the most up-to-date data
            // false = b0,
            // true = b1
            switch: false,
        }
    }
    pub fn init_boidee_random(&mut self, num: u32) {
        for _ in 0..num {
            let b = Boidee::random(&self.bounds);
            self.b0.push(b.clone());
            self.b1.push(b)
        }
        self.switch = false;
    }
    pub fn init_boidee(&mut self, num: u32) {
        for _ in 0..num {
            self.b0.push(Boidee::new());
            self.b1.push(Boidee::new());
        }
        // make sure we start knowing buffer 0 has the data
        self.switch = false;
    }
    pub fn step_draw<T: BoidCanvas>(&mut self, canvas: &mut T) {
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
            let new_boid = current.step(c, &self.bounds);
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
#[derive (Clone)]
struct Boidee {
    pos: Vector2,
    dir: f32,
    speed: f32,
}
impl fmt::Display for Boidee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos: ({},{}) bearing: {}° speed: {} )", self.pos.x, self.pos.y, self.dir, self.speed)
    }
}
impl Boidee {
    fn random(bounds: &(u32, u32)) -> Boidee {
        let mut rand = rand::thread_rng();
        Boidee {
            pos: Vector2::new(rand.gen::<f32>() * bounds.0 as f32, rand.gen::<f32>() * bounds.1 as f32),
            dir: rand::thread_rng().gen::<f32>() * 2.0,
            speed: rand::thread_rng().gen::<f32>() * 3.0,
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: 0.0,
            speed: 1.0,
        }
    }
    fn step(&self, flock: &Vec<Boidee>, bounds: &(u32, u32)) -> Boidee {
        let mut new_pos = self.pos + Vector2::new(self.dir.sin() * self.speed, self.dir.cos() * self.speed);
        new_pos.x = new_pos.x % bounds.0 as f32;
        new_pos.y = new_pos.y % bounds.1 as f32;
        if new_pos.x < 0.0 {
            new_pos.x += bounds.0 as f32;
        }
        if new_pos.y < 0.0 {
            new_pos.y += bounds.1 as f32;
        }
        Boidee {
            pos: new_pos,
            dir: self.dir.clone(),
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
