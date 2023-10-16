pub const SIZE_FACTOR: f32 = 8.0;

use crate::boidee::Boidee;
use crate::grid::Grid;
use crate::{LOCAL_SIZE, SCHEDULE_NANOS};
use std::slice::Chunks;
use std::time::{Instant, Duration};
use crate::BoidCanvas;
use std::f32::consts::PI;
pub struct Boid{
    bounds: (usize, usize),
    b0: Grid,
    b1: Grid,
    switch: bool,
    flock_scare: Option<f32>,
    cpus: usize
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
            //dt: Instant::now(),
            flock_scare: None,
            cpus: num_cpus::get()
        }
    }
    pub fn init_boidee_random(&mut self, num: usize) {
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
    pub fn flock_scare(&mut self, factor: Option<f32>){
        self.flock_scare = factor;
    }
    pub fn step_draw<T: BoidCanvas>(&mut self, canvas: &mut T) {
        let func_timer = Instant::now();
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
        // empty buffer
        let mut buffer: Vec<Boidee> = vec![];
        // flattened iterator over boidees
        // let flattened_refs: Vec<&Boidee> = c.cells.iter().flatten().flatten().collect();
        let flattened_refs: Vec<&Boidee> = c.cells.iter().flatten().flatten().collect();
        std::thread::scope(|scope: &std::thread::Scope<'_, '_>|{
            let thread_bounds = self.bounds.clone();
            let thread_flock_scare = self.flock_scare.clone();
            let mut handles = vec![];
            let f = (flattened_refs.len() as f32 / num_cpus::get() as f32).ceil() as usize;
            for task in flattened_refs.chunks(f){
                handles.push(scope.spawn(move ||{
                    let mut ret = vec![];
                    for boidee in task{
                        ret.push(boidee.step(c, &thread_bounds, thread_flock_scare));
                    }
                    ret
                }));
            }
            for handle in handles{
                buffer.append(&mut handle.join().unwrap());
            }
        });
        // for current in c.cells.clone().into_iter().flatten().flatten() {
        //     let new_boid= current.step(c, &self.bounds, self.flock_scare);
        //     buffer.push(new_boid);
        // }
        *b = Grid::from_vec(buffer, self.bounds, LOCAL_SIZE);
        // the buffers have been updated
        //self.dt = Instant::now();
        self.switch = !self.switch;
        //let draw_timer = Instant::now();
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
        let remaining = Duration::from_nanos(SCHEDULE_NANOS).checked_sub(func_timer.elapsed());
        if let Some(v) = remaining{
            std::thread::sleep(v);
            // println!("entire step_draw function was early by {:?}", v); // !!!
        }/*else{
            println!("entire step_draw function was late by {:?}", func_timer.elapsed() - Duration::from_nanos(SCHEDULE_NANOS)); // !!!
        }*/
        // println!("delta-time was {:?}", self.dt.elapsed()); // !!!
        
    }
}
