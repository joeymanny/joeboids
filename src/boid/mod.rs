use crate::boidee::Boidee;
use crate::grid::Grid;
use crate::{LOCAL_SIZE, SIZE_FACTOR, SCHEDULE_NANOS};
use std::time::{Instant, Duration};
use crate::BoidCanvas;
use std::f32::consts::PI;
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
            let (new_boid, what_it_sees) = current.step(c, &self.bounds);
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
            // println!("early! {:?}", v);
        }else{
            // println!("late! {:?}", self.dt.elapsed() - Duration::from_nanos(SCHEDULE_NANOS));
        }
        
    }
}