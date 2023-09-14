const SIZE_FACTOR: f32 = 8.0;
// TODO boid logic: turn toward flock center
use rand::Rng;
use std::f32::consts::PI;
use std::fmt;
use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
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
            let new_boid = current.step(c, &self.bounds, i);
            let h_sin = new_boid.dir.sin();
            let h_cos = new_boid.dir.cos();
            canvas
                .draw_triangle(
                    // tip: (sin * fac) + world
                    (
                        ((new_boid.dir.sin() * SIZE_FACTOR) + new_boid.pos.x) as i32,
                        ((new_boid.dir.cos() * SIZE_FACTOR) + new_boid.pos.y) as i32,
                    ),
                    // bottom left: (sin+90 * fac) + world
                    (
                        (((new_boid.dir + PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin)
                            + new_boid.pos.x) as i32,
                        (((new_boid.dir + PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos)
                            + new_boid.pos.y) as i32,
                    ),
                    // bottom right: (sin-90 * fac) + world
                    (
                        (((new_boid.dir - PI / 2.0).sin() * SIZE_FACTOR * 0.5 - h_sin)
                            + new_boid.pos.x) as i32,
                        (((new_boid.dir - PI / 2.0).cos() * SIZE_FACTOR * 0.5 - h_cos)
                            + new_boid.pos.y) as i32,
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
        let mut rand = rand::thread_rng();
        Boidee {
            pos: Vector2::new(
                rand.gen::<f32>() * bounds.0 as f32,
                rand.gen::<f32>() * bounds.1 as f32,
            ),
            dir: rand::thread_rng().gen::<f32>() * std::f32::consts::PI * 2.0,
            speed: rand::thread_rng().gen::<f32>() * 0.0, // THIS IS WHERE THE ISSUE IS
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: 0.0,
            speed: 1.0,
        }
    }
    fn step(&self, flock: &Vec<Boidee>, bounds: &(u32, u32), my_index: usize) -> Boidee {
        let mut flock_avg = Vector2::new(0.0, 0.0);
        //if flock.len() != 0 {
        //    for (i, boid) in flock.iter().enumerate() {
        //        if i != my_index {
        //            flock_avg = flock_avg + boid.pos;
        //        }
        //    }
        //    flock_avg = flock_avg / (flock.len() - 1) as f32;
            //    println!("I'm boid {} and i think the flock center is {}", my_index, flock_avg);
        //}
        let flock_avg = Vector2::new(300.0, 300.0);
        // TODO: they spin in place
            ref_to_total(
                ((flock_avg.y - self.pos.y) / (flock_avg.x - self.pos.x)).atan(),
                flock_avg
            );
        // boid steps forward
        let mut new_pos =
            self.pos + Vector2::new(self.dir.sin() * self.speed, self.dir.cos() * self.speed);

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
            dir: self.dir + 0.01,
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
#[derive(Clone, Copy)]
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

fn ref_to_total(mut me: f32, p: Vector2) -> f32 {
    println!("input angle: {} ({} pi)", me, me / PI);
    let f: f32;
    //let rs: bool;
    if p.x.is_sign_positive() {
        if p.y.is_sign_positive() {
            f = 0.0;
         //   rs = true;
        } else {
            f = 2.0 * PI;
       //     rs = false;
        }
    } else {
        if p.y.is_sign_positive() {
            f = PI;
     //       rs = false;
        } else {
            f = PI;
      //      rs = true;
        }
    }
    //me = me.abs();
    println!("output angle:{} ({})", f + me, (f + me) / PI);
    f + me
}
