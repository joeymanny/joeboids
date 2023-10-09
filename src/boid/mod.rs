use crate::grid::Grid;
use crate::{LOCAL_SIZE, SIZE_FACTOR, SCHEDULE_NANOS};
use std::time::{Instant, Duration};
use crate::BoidCanvas;
use std::f32::consts::PI;
use crate::vector2::Vector2;
use crate::angle::Angle;
pub struct Boid{
    bounds: (usize, usize),
    b0: Grid,
    b1: Grid,
    switch: bool,
    dt: Instant,
}
impl Boid {
    // fn update_grid(&mut self) -> Self{
    //     Self{
    //         grid: Grid::from_vec(if self.switch{&self.b1} else {&self.b0}, self.bounds, LOCAL_SIZE),
    //         ..self
    //     }
    // }
    pub fn new(bounds: (usize, usize)) -> Boid {
        Boid {
            bounds: bounds,
            b0: Grid::new(bounds, LOCAL_SIZE),
            b1: Grid::new(bounds, LOCAL_SIZE),
            // indicates which buffer has the most up-to-date data
            // false = b0,
            // true = b1
            switch: false,
            dt: Instant::now(),
        }
    }
    pub fn init_boidee_random(&mut self, num: u32) {
        let rand = Grid::random(num, self.bounds);
        self.b0 = rand.clone();
        self.b1 = rand;
        self.switch = false;

    }
    pub fn init_boidee(&mut self, num: u32) {
        let new = Grid::init_num(num, self.bounds);
        self.b0 = new.clone();
        self.b1 = new;
        // make sure we start knowing buffer 0 has the data
        self.switch = false;
    }
    // calls func after calculating but before rendering
    pub fn step_draw<T: BoidCanvas>(&mut self, canvas: &mut T) {
        // target buffer
        let b: &mut Grid;
        // buffer containing most up-to-date boids
        let c: &Grid;
        if self.switch {
            b = &mut self.b0;
            c = &self.b1;
        } else {
            b = &mut self.b1;
            c = &self.b0;
        }
        // update grid from buffer
        // update all boids
        let mut buffer: Vec<Boidee> = vec![];
        for current in c.cells.iter().flatten().flatten() {
            let (new_boid, what_it_sees) = current.step(b, &self.bounds);
            buffer.push(new_boid);
            if let Some(v) = what_it_sees {
                for b in v{
                    canvas.draw_triangle(b.pos.clone().into(), current.pos.into(), b.pos.into()).unwrap();
                }
            }
        }
        *b = Grid::from_vec(buffer, self.bounds, LOCAL_SIZE);
        // the buffers have been updated
        self.dt = Instant::now();
        self.switch = !self.switch;
        for new_boid in c.cells.iter().flatten().flatten() {
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
        let remaining = Duration::from_nanos(SCHEDULE_NANOS).checked_sub(self.dt.elapsed());
        if let Some(v) = remaining{
            std::thread::sleep(v);
            println!("early! {:?}", v);
        }else{
            println!("late! {:?}", self.dt.elapsed() - Duration::from_nanos(SCHEDULE_NANOS));
        }
        
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pos: Vector2,
    dir: Angle,
    speed: f32,
    randscope: usize,
    rand: f32,
    chosen: bool
}
impl std::fmt::Display for Boidee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: ({},{}) bearing: {}° speed: {} )",
            self.pos.x, self.pos.y, self.dir, self.speed
        )
    }
}
impl Boidee {
    fn random(bounds: (usize, usize)) -> Boidee {
        let mut r = rand::thread_rng();
        Boidee {
            pos: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            dir: Angle::new(r.gen::<f32>() * (PI * 2.0)),
            speed: 1.0 - (r.gen::<f32>() * 0.3),
            randscope: 0,
            rand: 0.0,
            chosen: false
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: Angle::new(0.0),
            speed: 2.0,
            randscope: 0,
            rand: 0.0,
            chosen: false,
        }
    }
    fn step(
        &self,
        flock: &Grid,
        bounds: &(usize, usize),
    ) -> (Boidee, Option<Vec<Boidee>>) {
        let mut r = rand::thread_rng();
        let mut new_dir = self.dir;
        let mut new_pos = Vector2::new(0.0, 0.0);
        let new_randscope;
        let new_rand;
        if self.randscope <= 0 {
            new_randscope = (r.gen::<f32>() * MAX_RAND_SCOPE as f32) as usize;
            new_rand = (r.gen::<f32>() - 0.5) / 10.0;
        } else {
            new_dir = new_dir + self.rand;
            new_randscope = self.randscope - 1;
            new_rand = self.rand;
        }
        let mut local_avg = Vector2::new(0.0, 0.0);
        let mut local_num = 0;
        let mut local_dir = Angle::new(0.0);
        let mut too_close_p = Vector2::new(0.0, 0.0);
        let mut too_close_n = 0;
        let mut amogus: Option<Vec<Boidee>> = None;
        let neighbors = flock.get_cell_neighbors(&self);
        if self.chosen{
            amogus = Some(Vec::new());
        }
        for fren in neighbors {
            if fren != *self {
                let dist = (fren.pos - self.pos).abs();
                if dist <= LOCAL_SIZE {
                    if dist <= TOO_CLOSE {

                        too_close_p = too_close_p + fren.pos;
                        too_close_n += 1;

                    }
                    local_dir =  (fren.dir) + local_dir;
                    local_avg = local_avg + fren.pos;
                    local_num += 1;
                    if self.chosen{
                        amogus.as_mut().unwrap().push(fren);
                    }
                }
            }
        }
        // all adjustments that rely on local group averages
        // new_pos = self.pos;
        if local_num != 0 {
            if too_close_n != 0 {
                too_close_p = too_close_p / too_close_n as f32;
                // avoid locals too close
                new_pos = (too_close_p - self.pos) / -10.0;
            }
            // local_avg = local_avg / local_num as f32;
            // go towards center of local cluster
            // if local_avg != Vector2::new(0.0, 0.0){
                // new_pos = (local_avg - self.pos) / 1000.0;
            // }
            // try face local average
            new_dir = Angle::new(face(new_dir, local_dir));
        }

        // boid steps forward
        new_pos =
            new_pos + self.pos + Vector2::new(new_dir.cos() * self.speed, new_dir.sin() * self.speed);

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
            new_dir = new_dir + (2.0 * PI) ;
        }
        (Boidee {
            pos: new_pos,
            dir: new_dir,
            speed: self.speed,
            randscope: new_randscope,
            rand: new_rand,
            chosen: self.chosen
        }, amogus)
    }
}
