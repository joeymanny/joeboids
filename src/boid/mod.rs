pub const SIZE_FACTOR: f32 = 8.0;

use crate::boidee::Boidee;
use crate::grid::Grid;
use crate::vector2::Vector2;
use crate::{LOCAL_SIZE, SCHEDULE_NANOS};
use std::time::{Instant, Duration};
use crate::BoidCanvas;
pub use crate::boidee::TargetType;

pub struct Boid{
    bounds: ((f32, f32), (f32, f32)),
    b0: Grid,
    b1: Grid,
    switch: bool,
    flock_scare: Option<f32>,
    cpus: usize,
    tiny: Option<f32>,
    avg_time: f32,
    avg_times: usize
}
impl Boid {
    // fn update_grid(&mut self) -> Self{
    //     Self{
    //         grid: Grid::from_vec(if self.switch{&self.b1} else {&self.b0}, self.bounds, LOCAL_SIZE),
    //         ..self
    //     }
    // }
    pub fn new(bounds: ((f32, f32), (f32, f32))) -> Boid {
        Boid {
            bounds: bounds,
            b0: Grid::new(bounds.0, bounds.1, LOCAL_SIZE),
            b1: Grid::new(bounds.0, bounds.1, LOCAL_SIZE),
            // indicates which buffer has the most up-to-date data
            // false = b0,
            // true = b1
            switch: false,
            //dt: Instant::now(),
            flock_scare: None,
            cpus: num_cpus::get(),
            tiny: None,
            avg_time: 0.0,
            avg_times: 0
        }
    }
    pub fn set_tiny(&mut self, state: Option<f32>){
        self.tiny = state;
    }
    pub fn init_boidee_random(&mut self, num: usize) {
        let rand = Grid::random(num, self.bounds.0, self.bounds.1);
        self.b0 = rand.clone();
        self.b1 = rand;
        self.switch = false;

    }
    pub fn init_boidee(&mut self, num: u32) {
        let new = Grid::init_num(num);
        self.b0 = new.clone();
        self.b1 = new;
        // make sure we start knowing buffer 0 has the data
        self.switch = false;
    }
    pub fn flock_scare(&mut self, factor: Option<f32>){
        self.flock_scare = factor;
    }
    pub fn step_draw_target<T: BoidCanvas>(&mut self, canvas: &mut T, target: (f32, f32), target_type: crate::boidee::TargetType){
        Self::step_draw_generic_function(self, canvas, Some((Vector2{x: target.0, y: target.1}, target_type)))
    }
    pub fn step_draw<T: BoidCanvas>(&mut self, canvas: &mut T){
        Self::step_draw_generic_function(self, canvas, None,)
    }
    fn step_draw_generic_function<T: BoidCanvas>(&mut self, canvas: &mut T, target: Option<(Vector2, crate::boidee::TargetType)>) {
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
        // empty buffer
        let mut buffer: Vec<Boidee> = vec![];
        // flattened Vec over boidees
        let flattened_refs: Vec<&Boidee> = c.iterate_flattened().collect();
        std::thread::scope(|scope|{
            let thread_bounds = self.bounds;
            let thread_flock_scare = self.flock_scare;
            let thread_target = target;
            let mut handles = vec![];
            for task in flattened_refs.chunks((flattened_refs.len() as f32 / self.cpus as f32).ceil() as usize){
                handles.push(scope.spawn(move ||{
                    let mut ret = vec![];
                    for boidee in task{
                        ret.push(boidee.step(c.get_cell_neighbors(&boidee), thread_bounds.0, thread_bounds.1, thread_flock_scare, thread_target));
                    }
                    ret
                }));
            }
            for handle in handles{
                buffer.append(&mut handle.join().unwrap());
            }
        });
        *b = Grid::from_vec(buffer, LOCAL_SIZE);
        // the buffers have been updated
        //self.dt = Instant::now();
        self.switch = !self.switch;
        let tinyness = if let Some(v) = self.tiny{
            v
        }else {1.0};
        for new_boid in c.iterate_flattened() {
            let direction = new_boid.velocity.normalized();
            canvas
                .draw_triangle(
                    (
                        ((direction.x * SIZE_FACTOR * tinyness) + new_boid.position.x) as i32,
                        ((direction.y * SIZE_FACTOR * tinyness) + new_boid.position.y) as i32,
                    ),
                    // bottom left: (sin+90 * fac) + world
                    (
                        ((-direction.y * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.x) as i32,
                        ((direction.x * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.y) as i32,
                    ),
                    // bottom right: (sin-90 * fac) + world
                    (
                        ((direction.y * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.x) as i32,
                        ((-direction.x * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.y) as i32,
                    ),
                )
                .unwrap();
        }
        let remaining = Duration::from_nanos(SCHEDULE_NANOS).checked_sub(func_timer.elapsed());
        if let Some(v) = remaining{
            std::thread::sleep(v);
            println!("entire step_draw function was early by {:?}", v); // !!!
            self.avg_time -= v.as_secs_f32();
            self.avg_times += 1;
            println!("average: {} seconds", self.avg_time / self.avg_times as f32);
        }else{
            let lateness = func_timer.elapsed() - Duration::from_nanos(SCHEDULE_NANOS);
            println!("entire step_draw function was late by {:?}", lateness); // !!!
            self.avg_time += lateness.as_secs_f32();
            self.avg_times += 1;
            println!("average: {} seconds", self.avg_time / self.avg_times as f32);

        }       
    }
}
