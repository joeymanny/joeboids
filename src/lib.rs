const SIZE_FACTOR: f32 = 8.0;
const TOO_CLOSE: f32 = 15.0;
const LOCAL_SIZE: f32 = 50.0;
const MAX_RAND_SCOPE: f32 = 3.0;

use rand::Rng;
use std::f32::consts::PI;
use std::fmt;
use std::iter::zip;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::time::Instant;
use std::ops::Deref;
use std::fmt::Display;
use std::ops::Rem;
use std::cmp::Ordering;
pub trait BoidCanvas {
    fn draw_triangle(
        &mut self,
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
    ) -> Result<(), String>;
}
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
        let dt = self.dt.elapsed().as_secs_f32();
        let mut buffer: Vec<Boidee> = vec![];
        for current in c.cells.iter().flatten().flatten() {
            let (new_boid, what_it_sees) = current.step(b, &self.bounds, dt);
            buffer.push(new_boid);
            if let Some(v) = what_it_sees {
                for b in v{
                    canvas.draw_triangle(b.pos.clone().into(), current.pos.into(), b.pos.into()).unwrap();
                }
            }
        }
        let x = 1;
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
        
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Boidee {
    pos: Vector2,
    dir: Angle,
    speed: f32,
    agility: f32,
    randscope: f32,
    rand: f32,
    chosen: bool
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
    fn random(bounds: (usize, usize)) -> Boidee {
        let mut r = rand::thread_rng();
        Boidee {
            pos: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            dir: Angle::new(r.gen::<f32>() * (PI * 2.0)),
            speed: 200.0 - (r.gen::<f32>() * 10.0),
            agility: 0.01,
            randscope: 0.0,
            rand: 0.0,
            chosen: false
        }
    }
    fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: Angle::new(0.0),
            speed: 5.0,
            agility: 0.5,
            randscope: 0.0,
            rand: 0.0,
            chosen: false,
        }
    }
    fn step(
        &self,
        flock: &Grid,
        bounds: &(usize, usize),
        dt: f32,
    ) -> (Boidee, Option<Vec<Boidee>>) {
        let mut r = rand::thread_rng();
        let mut new_dir = self.dir;
        let mut new_pos = Vector2::new(0.0, 0.0);
        let new_randscope;
        let new_rand;
        if self.randscope <= 0.0 {
            new_randscope = r.gen::<f32>() * MAX_RAND_SCOPE;
            new_rand = (r.gen::<f32>() - 0.5) * 1000.0;
        } else {
            new_dir = (new_dir + (self.rand * dt * self.agility)) % (2.0 * PI);
            new_randscope = self.randscope - dt;
            new_rand = self.rand;
            new_dir = new_dir % (2.0 * PI);
            if new_dir < 0.0 {
                new_dir = new_dir + (2.0 * PI);
            }
        }
        let mut local_avg = Vector2::new(0.0, 0.0);
        let mut local_num = 0;
        let mut local_dir = 0.0;
        let mut too_close_p = Vector2::new(0.0, 0.0);
        let mut too_close_n = 0;
        let mut amogus: Option<Vec<Boidee>> = None;
        let neighbors = flock.get_cell_neighbors(&self);
        if self.chosen{
            amogus = Some(neighbors.clone());
        }
        for fren in neighbors {
            if fren != *self {
                let dist = (fren.pos - self.pos).abs();
                if dist <= LOCAL_SIZE {
                    if dist <= TOO_CLOSE {
                        too_close_p = too_close_p + fren.pos;
                        too_close_n += 1;
                    }
                    local_dir =  (*fren.dir) + local_dir;
                    local_avg = local_avg + fren.pos;
                    local_num += 1;
                }
            }
        }
        // all adjustments that rely on local group averages
        if local_num != 0 {
            if too_close_n != 0 {
                too_close_p = too_close_p / too_close_n as f32;
                // avoid locals too close
                new_dir = new_dir + (self.agility * dt * 0.0 * avoid_point(*new_dir, too_close_p));
            }
            local_avg = local_avg / local_num as f32;
            local_dir = local_dir / local_num as f32;
            // go towards center of local cluster
            new_dir = new_dir + (self.agility * 0.0 * dt * face_point(*new_dir, local_avg));
            // try face local average
            new_dir = new_dir + (self.agility * 1000.0 * dt * face(*new_dir, local_dir));
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
            new_dir = new_dir + (2.0 * PI) ;
        }
        (Boidee {
            pos: new_pos,
            dir: new_dir,
            speed: self.speed,
            agility: self.agility,
            randscope: new_randscope,
            rand: new_rand,
            chosen: self.chosen
        }, amogus)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Angle (f32);
impl Angle {
    fn new (mut x: f32) -> Angle {
        x  %= 2.0 * PI;
        if x < 0.0{
            Angle((2.0 * PI) + x)
        }else{
            Angle(x)
        }
    }
}
impl Add<f32> for Angle {
    type Output = Self;
    fn add<>(self, other: f32) -> Self::Output {
            Self((self.0 + other) % (2.0 * PI))
    }
}
impl Sub<f32> for Angle {
    type Output = Self;
    fn sub(self, other: f32) -> Self{
        let mut ans = self.0 - other;
        ans %= PI * 2.0;
        if ans < 0.0 {
            Self(ans + 2.0 * PI)
        }else{
            Self(ans)
        }
    }
}
impl Deref for Angle{
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
        "{}",
        self.0
        )
    }
}
impl Rem<f32> for Angle{
    type Output = Self;

    fn rem(self, modulus: f32) -> Self::Output {
        Angle(self.0 % modulus)
    }
}
impl PartialEq<f32> for Angle {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}
impl PartialOrd<f32> for Angle{
    fn partial_cmp(&self, other: &f32) -> Option<Ordering>{
        Some(self.0.total_cmp(other))
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector2 {
    x: f32,
    y: f32,
}
impl From<Vector2> for (i32, i32){
    fn from(val: Vector2) -> Self{
        (val.x.floor() as i32, val.y.floor() as i32)
    }
}
impl Vector2 {
    fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    fn abs(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    // fn normalized(self) -> Self {
        // let fac = 1.0 / self.abs();
        // Vector2 {
            // x: self.x * fac,
            // y: self.y * fac,
        // }
    // }
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
#[derive(Clone, Debug)]
struct Grid{
    max: (usize, usize),
    cells: Vec<Vec<Vec<Boidee>>>,
    fac: f32,
}
impl Grid{
    fn new(max: (usize,usize), fac: f32) -> Grid{
        let cells: Vec<Vec<Vec<Boidee>>> = init_grid_vec(max, fac);
        Self { max, cells, fac }
    }
    fn from_vec(data: Vec<Boidee>, max: (usize, usize), fac: f32) -> Grid{ // DONE!
        // make an array of cells of the right size
        // populate the Vec's of the cells with references to the data
        // profit

        // Vec of Vec's of Vec's of Boidees
        //      0: 0|1|2|3|4
        //      1: 0|1|2|3|4 <-- each of these is a Vec of Boidee's
        //      2: 0|1|2|3|4
        //      3: 0|1|2|3|4
        //      4: 0|1|2|3|4

        // empty 3D array (3rd dimension for Boidees)
        let mut buf: Vec<Vec<Vec<Boidee>>> = init_grid_vec(max, fac);
        // fill them with data
        // this will panic is max is too small, so make sure max isn't too small
        for boidee in data{
            let adj_x = (boidee.pos.x / fac).floor() as usize;
            let adj_y = (boidee.pos.y / fac).floor() as usize;
            buf[adj_x][adj_y].push(boidee);
        }
        Self { max, cells: buf, fac }
    }
    fn get_cell_neighbors(&self, sub: &Boidee) -> Vec<Boidee>{
        // we are assuming that all Boidees have positions within the max
        // we can assume this because these (should be) both coordinated by Boid
        // just don't mess up Boid and it'll be fine
        let mut rtrn: Vec<Boidee> = vec![];
        let x_adj: usize = (sub.pos.x / self.fac as f32).floor() as usize;
        let y_adj: usize = (sub.pos.y / self.fac as f32).floor() as usize;
        let sub_cell = self.cells[x_adj][y_adj].clone();
        let x_0 = x_adj <= 1;
        let y_0 = y_adj <= 1;
        let y_max = y_adj >= ((self.max.1 - 1) as f32 / self.fac).floor() as usize;
        let x_max = x_adj >= ((self.max.0 - 1) as f32 / self.fac).floor() as usize;
        
        //left
            //upper left
            //lower left
        //right
            //uppper right
            //lower left
        //up
        //down
        if !x_0{
            // left
            rtrn.append(&mut self.cells[x_adj - 1][y_adj].clone());
            if !y_0{
                //upper left
                rtrn.append(&mut self.cells[x_adj - 1][y_adj - 1].clone());
            }
            if !y_max{
                // lower left
                rtrn.append(&mut self.cells[x_adj - 1][y_adj + 1].clone());
            }
        }
        if !x_max{
            //right
            rtrn.append(&mut self.cells[x_adj + 1][y_adj].clone());
            if !y_0{
                // upper right
                rtrn.append(&mut self.cells[x_adj + 1][y_adj - 1].clone());
            }
            if !y_max{
                // lower right
                rtrn.append(&mut self.cells[x_adj + 1][y_adj + 1].clone());
            }
        }
        if !y_max{
            // down
            rtrn.append(&mut self.cells[x_adj][y_adj + 1].clone());
        }
        if !y_0{
            // up
            rtrn.append(&mut self.cells[x_adj][y_adj - 1].clone());
        }
        // we also need our own cell of course
        rtrn.append(&mut sub_cell.clone());

        rtrn
    }
    fn random(num: u32, bounds: (usize, usize)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..(num
        ){
            v.push(Boidee::random(bounds));
        }
        v[0].chosen = true; // TODO -------------------------------------- REMOVE THIS!
        Grid::from_vec(v, bounds, LOCAL_SIZE)

    }
    fn init_num(num: u32, bounds: (usize, usize)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..num{
            v.push(Boidee::new());
        }
        Grid::from_vec(v, bounds, LOCAL_SIZE)
    }
}
fn init_grid_vec(max: (usize, usize), fac: f32) -> Vec<Vec<Vec<Boidee>>> {
    let mut x_array: Vec<Vec<Vec<Boidee>>> = Vec::new();
    for _ in 0..((max.0 as f32 / fac).floor() as usize){
        let mut y_array: Vec<Vec<Boidee>> = Vec::new();
        for _ in 0..((max.1 as f32 / fac).floor() as usize){
            y_array.push(Vec::new());
        }
        x_array.push(y_array);
    }
    x_array
}