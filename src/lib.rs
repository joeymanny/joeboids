const SIZE_FACTOR: f32 = 8.0;
// TODO boid logic: turn toward flock center

use rand::Rng;
use std::f32::consts::PI;
use std::fmt;
use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    ) -> Result<(), String>;
}
pub struct Boid {
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
        let b;
        // buffer containing most up-to-date boids
        let c;
        if self.switch {
            b = &mut self.b0;
            c = &self.b1;
        } else {
            b = &mut self.b1;
            c = &self.b0;
        }
        let mut flock_avg = Vector2::new(0.0, 0.0);
        if b.len() != 0 {
            for boid in b.iter() {
                flock_avg = flock_avg + boid.pos;
            }
            flock_avg = flock_avg / b.len() as f32;
            canvas.draw_triangle(
                (
                    flock_avg.x.round() as i32 - 3,
                    flock_avg.y.round() as i32 - 3,
                ),
                (
                    flock_avg.x.round() as i32 + 3,
                    flock_avg.y.round() as i32 - 3,
                ),
                (flock_avg.x.round() as i32, flock_avg.y.round() as i32 + 3),
            );
        }
        for (i, (current, buffer)) in zip(c, b).enumerate() {
            let new_boid = current.step(c, &self.bounds, i, flock_avg);
            let h_sin = new_boid.dir.sin();
            let h_cos = new_boid.dir.cos();
            canvas
                .draw_triangle(
                    // tip: (sin * fac) + world
                    (
                        ((new_boid.dir.cos() * SIZE_FACTOR) + new_boid.pos.x) as i32,
                        ((new_boid.dir.sin() * SIZE_FACTOR) + new_boid.pos.y) as i32,
                    ),
                    // bottom left: (sin+90 * fac) + world
                    (
                        (((new_boid.dir + PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos) + new_boid.pos.x) as i32,
                        (((new_boid.dir + PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin) + new_boid.pos.y) as i32,
                    ),
                    // bottom right: (sin-90 * fac) + world
                    (
                        (((new_boid.dir - PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos) + new_boid.pos.x) as i32,
                        (((new_boid.dir - PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin)+ new_boid.pos.y) as i32,
                    ),
                )
                .unwrap();
            *buffer = new_boid;
        }

        self.switch = !self.switch;
    }
}
#[derive(Clone)]
struct Boidee {
    pos: Vector2,
    dir: f32,
    speed: f32,
    agility: f32,
}
impl fmt::Display for Boidee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos: ({},{}) bearing: {}° speed: {} )",
            self.pos.x, self.pos.y, self.dir, self.speed
        )
    }
}
impl Boidee {
    fn random(bounds: &(u32, u32)) -> Boidee {
        let mut r = rand::thread_rng();
        let mut ag = r.gen::<f32>();
        Boidee {
            pos: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            dir: r.gen::<f32>() * PI * 2.0,
            speed: 0.1 + r.gen::<f32>() ,
            agility: 0.007
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: 0.0,
            speed: 5.0,
            agility: 0.5,
        }
    }
    fn step(&self, flock: &Vec<Boidee>, bounds: &(u32, u32), my_index: usize, flock_avg: Vector2) -> Boidee {
        // TODO: they spin in place
            let mut new_dir = self.dir + (1.0 *  (self.agility * (face_point(self.dir, flock_avg - self.pos))));
            new_dir = new_dir + (0.7 * (self.agility * avoid_point(new_dir, Vector2::new(200.0, 400.0) - self.pos)));
        // boid steps forward
        let mut new_pos =
            self.pos + Vector2::new(new_dir.cos() * self.speed, new_dir.sin() * self.speed);

        // all modifications to pos should be done before this point
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
            dir: new_dir,
            speed: self.speed.clone(),
            agility: self.agility.clone()
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
#[derive(Clone, Copy, Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    fn normalized(self) -> Self {
        let fac = 1.0 / (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        Vector2 {
            x: self.x * fac,
            y: self.y * fac,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn avoid_point(curr: f32, point: Vector2) -> f32 {
    let point = point * -1.0;
    face_point(curr, point)
}
fn face_point(curr: f32, point: Vector2) -> f32 {
    let wish = atan2_to_total(point.y.atan2(point.x));
    let mut means = wish - curr;
    // check if theres a closer way going to opposite direction
    if means > PI {
        means = (2.0 * PI) - means
    }
    if means < (-1.0 * PI) {
        means = (2.0 * PI) + means
    }
    means % (2.0 * PI)

}
fn atan2_to_total(n: f32) -> f32 {
    if n.is_sign_negative() {
        (2.0 * PI) + n
    } else {
        n
    }
}
