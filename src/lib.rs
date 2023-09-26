const SIZE_FACTOR: f32 = 8.0;
const TOO_CLOSE: f32 = 15.0;
const LOCAL_SIZE: f32 = 50.0;
const MAX_RAND_SCOPE: f32 = 4.0;

use rand::Rng;
use std::f32::consts::PI;
use std::fmt;
use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::time::Instant;
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
    dt: Instant,
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
            dt: Instant::now(),
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
        self.step_fn_draw(canvas, ||{})
    }
    // calls func after calculating but before rendering
    pub fn step_fn_draw<T: BoidCanvas, F: FnOnce() -> ()>(&mut self, canvas: &mut T, func: F) {
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
            //canvas.draw_triangle(
            //    (
            //        flock_avg.x.round() as i32 - 3,
            //        flock_avg.y.round() as i32 - 3,
            //    ),
            //    (
            //        flock_avg.x.round() as i32 + 3,
            //        flock_avg.y.round() as i32 - 3,
            //    ),
            //    (flock_avg.x.round() as i32, flock_avg.y.round() as i32 + 3),
            //);
        }
        // update all boids
        let dt = self.dt.elapsed().as_secs_f32();
        for (i, (current, buffer)) in zip(c, b).enumerate() {
            let new_boid = current.step(c, &self.bounds, i, flock_avg, dt);
            *buffer = new_boid
        }
        func();
        for new_boid in c {
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
                        (((new_boid.dir + PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos)
                            + new_boid.pos.x) as i32,
                        (((new_boid.dir + PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin)
                            + new_boid.pos.y) as i32,
                    ),
                    // bottom right: (sin-90 * fac) + world
                    (
                        (((new_boid.dir - PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos)
                            + new_boid.pos.x) as i32,
                        (((new_boid.dir - PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin)
                            + new_boid.pos.y) as i32,
                    ),
                )
                .unwrap();
        }
        self.dt = Instant::now();
        self.switch = !self.switch;
    }
}
#[derive(Clone)]
struct Boidee {
    pos: Vector2,
    dir: f32,
    speed: f32,
    agility: f32,
    randscope: f32,
    rand: f32,
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
            dir: r.gen::<f32>() * (PI * 2.0),
            speed: 200.0 - (r.gen::<f32>() * 10.0),
            agility: 0.01,
            randscope: 0.0,
            rand: 0.0,
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: 0.0,
            speed: 5.0,
            agility: 0.5,
            randscope: 0.0,
            rand: 0.0,
        }
    }
    fn step(
        &self,
        flock: &Vec<Boidee>,
        bounds: &(u32, u32),
        my_index: usize,
        flock_avg: Vector2,
        dt: f32,
    ) -> Boidee {
        let mut r = rand::thread_rng();
        let mut new_dir = self.dir;
        let mut new_pos = Vector2::new(0.0, 0.0);
        let new_randscope;
        let new_rand;
        if self.randscope <= 0.0 {
            new_randscope = (r.gen::<f32>() * MAX_RAND_SCOPE);
            new_rand = (r.gen::<f32>() - 0.5) * 1000.0;
        } else {
            new_dir = (new_dir + (self.rand * dt * self.agility)) % (2.0 * PI);
            new_randscope = self.randscope - dt;
            new_rand = self.rand;
            new_dir = new_dir % (2.0 * PI);
            if new_dir < 0.0 {
                new_dir = (2.0 * PI) + new_dir;
            }
        }
        let mut local_avg = Vector2::new(0.0, 0.0);
        let mut local_num = 0;
        let mut local_dir = 0.0;
        let mut too_close_p = Vector2::new(0.0, 0.0);
        let mut too_close_n = 0;
        for (i, fren) in flock.iter().enumerate() {
            if i != my_index {
                let dist = (fren.pos - self.pos).abs();
                if dist <= LOCAL_SIZE {
                    if dist <= TOO_CLOSE {
                        too_close_p = too_close_p + fren.pos;
                        too_close_n += 1;
                    }
                    local_dir += fren.dir;
                    local_avg = local_avg + fren.pos;
                    local_num += 1;
                }
            }
        }
        // all adjustments that rely on local group averages
        if local_num != 0 {
            if too_close_n != 0 {
                too_close_p = (too_close_p / too_close_n as f32);
                // avoid locals too close
<<<<<<< HEAD
                new_dir = new_dir + (self.agility * 5.0 * avoid_point(new_dir, too_close_p));
=======
                new_pos = new_pos - (((too_close_p - self.pos) * (1.0 / (too_close_p.abs() - 0.01))) * 0.5);
                new_dir = new_dir + (self.agility * 600.0 * dt * avoid_point(self.dir, too_close_p - self.pos));
>>>>>>> ded40f4473136c45ce06165835d64cd23aca8ae6
            }
            local_avg = local_avg / local_num as f32;
            local_avg = local_avg;
            local_dir = local_dir / local_num as f32;
            // go towards center of local cluster
<<<<<<< HEAD
            new_dir = new_dir + (self.agility * 3.0 * face_point(new_dir, local_avg));
            // try face local average
            new_dir = new_dir + (self.agility * 8.0 * face(new_dir, local_dir));
=======
            new_dir = new_dir + (self.agility * 80.0 * dt * face_point(self.dir, local_avg - self.pos));
            // try face local average
            new_dir = new_dir + (self.agility * 800.0 * dt * face(self.dir, local_dir));
>>>>>>> ded40f4473136c45ce06165835d64cd23aca8ae6
        }

        // boid steps forward
        new_pos =
            new_pos + self.pos + Vector2::new(new_dir.cos() * self.speed * dt, new_dir.sin() * self.speed * dt);

        // all modifications to pos & dir should be done before this point
        new_pos.x = new_pos.x % bounds.0 as f32;
        new_pos.y = new_pos.y % bounds.1 as f32;
        if new_pos.x < 0.0 {
            new_pos.x += bounds.0 as f32;
        }
        if new_pos.y < 0.0 {
            new_pos.y += bounds.1 as f32;
        }
        new_dir = new_dir % (2.0 * PI);
        if new_dir < 0.0 {
            new_dir = (2.0 * PI) + new_dir;
        }
        Boidee {
            pos: new_pos,
            dir: new_dir,
            speed: self.speed.clone(),
            agility: self.agility.clone(),
            randscope: new_randscope,
            rand: new_rand,
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
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    fn abs(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn normalized(self) -> Self {
        let fac = 1.0 / self.abs();
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
fn face(curr: f32, wish: f32) -> f32 {
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
fn avoid_point(curr: f32, point: Vector2) -> f32 {
    let point = point * -1.0;
    face_point(curr, point)
}
fn face_point(curr: f32, point: Vector2) -> f32 {
    let wish = atan2_to_total(point.y.atan2(point.x));
    let mut means = wish - curr;
    // check if theres a closer way going to opposite direction
    if means > PI {
        means = (-2.0 * PI) + means
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
