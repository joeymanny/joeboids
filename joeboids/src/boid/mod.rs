
use rayon::prelude::*;

// how big to draw the boidees
// tinyness will scale this value
const SIZE_FACTOR: f32 = 8.0;

const DEFAULT_VISUAL_RANGE: f32 = 40.0;

use crate::boidee::Boidee;
use crate::grid::Grid;
use std::time::{Instant, Duration};
use crate::BoidCanvas;

pub struct Boid{
    bounds: ((f32, f32), (f32, f32)),
    b0: Grid,
    b1: Grid,
    switch: bool,
    flock_scare: Option<f32>,
    tiny: Option<f32>,
    schedule: Option<std::time::Duration>,
    visual_range: f32,
    #[cfg(feature = "print_timings")]
    avg_time: f32,
    #[cfg(feature = "print_timings")]
    avg_times: usize
}
impl Boid {
    /// Returns a new [`Boid`] with the bounds specified. Fields on this struct reflect the current working
    /// state (i.e. which `Grid` of [`Boidee`]s to use) and so are private. The `Grid`s start empty, and
    /// must be poulated with [init_boidee](crate::boid::Boid::init_boidees) or [init_boidee_random](crate::boid::Boid::init_boidee_random)
    pub fn new(bounds: ((f32, f32), (f32, f32)), schedule: Option<Duration>) -> Boid {
        let empty_grid = Grid::new(bounds.0, bounds.1, DEFAULT_VISUAL_RANGE);
        Boid {
            bounds: bounds,
            b0: empty_grid.clone(),// ew
            b1: empty_grid,
            switch: false,
            //dt: Instant::now(),
            flock_scare: None,
            tiny: None,
            schedule,
            visual_range: DEFAULT_VISUAL_RANGE,
            #[cfg(feature = "print_timings")]
            avg_time: 0.0,
            #[cfg(feature = "print_timings")]
            avg_times: 0
        }
    }
        /// Updates then displays the [`Boidee`]s held by the [`Boid`]. If it gets done before [`schedule`](Boid::schedule), it
    /// will [`sleep`](std::thread::sleep) for the remaining time. Also prints timing info if the `print_timings` feature is enabled.
    pub fn step_on_schedule<T: BoidCanvas>(&mut self, canvas: &mut T, target: Option<((f32, f32), crate::TargetType)>) {
        let func_timer = Instant::now();

        self.raw_step(target);

        self.raw_draw(canvas);

        if let Some(duration) = self.schedule{
            let remaining = duration.checked_sub(func_timer.elapsed());
            if let Some(v) = remaining{
                std::thread::sleep(v);
                #[cfg(feature = "print_timings")]
                {println!("entire step_draw function was early by {:?}", v); // !!!
                self.avg_time -= v.as_secs_f32();
                self.avg_times += 1;
                println!("average: {} seconds", self.avg_time / self.avg_times as f32);}
            }else{
                #[cfg(feature = "print_timings")]
                {let lateness = func_timer.elapsed() - Duration::from_nanos(16_666_666);
                println!("entire step_draw function was late by {:?}", lateness); // !!!
                self.avg_time += lateness.as_secs_f32();
                self.avg_times += 1;
                println!("average: {} seconds", self.avg_time / self.avg_times as f32);}
            }
        }
    }

    /// Draws the current state of the [`Boid`] to the `canvas` passed in. Does not honor [`schedule`](Boid::schedule) or the `print_timings` feature.
    pub fn raw_draw<T: BoidCanvas>(&self, canvas: &mut T){
        let tinyness = if let Some(v) = self.tiny{
            v
        }else {1.0};
        // buffer containing most up-to-date boids.
        let c: &Grid;
        if self.switch {
            c = &self.b1;
        } else {
            c = &self.b0;
        }
        // 8.0 because it looks good when tinyness is 1.0
        for new_boid in c.iterate_flattened() {
            let direction = new_boid.velocity.normalized();
            canvas
                // same idea as getting a line perpendicular to another line
                // switch x and y and make on negative
                .draw_triangle(
                    (
                        ((direction.x * SIZE_FACTOR * tinyness) + new_boid.position.x),
                        ((direction.y * SIZE_FACTOR * tinyness) + new_boid.position.y),
                    ),
                    (
                        ((-direction.y * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.x),
                        ((direction.x * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.y),
                    ),
                    (
                        ((direction.y * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.x),
                        ((-direction.x * SIZE_FACTOR * tinyness) / 2.0 + new_boid.position.y),
                    ),
                )
                .unwrap();
        }
    }

    /// Updates the [`Boid`] one step. Does not consider the schedule: running this in a loop will
    /// result in variable timing. Use [`step_on_schedule`](Boid::step_on_schedule) for realtime applications.
    /// Does not honor [`schedule`](Boid::schedule) or the `print_timings` feature.
    pub fn raw_step(&mut self, target: Option<((f32, f32), crate::TargetType)>){
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
        cfg_if::cfg_if! {
            if #[cfg(feature = "visualize_neighbors")] { // IF VISUALIZE NEIGHBORS FEATURE IS SET ------------------------------
                let result: Vec<(Boidee, Option<Vec<Boidee>>)> = c.iterate_flattened().collect::<Vec<&Boidee>>()
                .into_par_iter().map(
                    |boid|{
                        let neighbors = c.get_cell_neighbors(boid);
                        let returned_neighbors = if boid.chosen{
                            Some(neighbors.clone())
                        }else{
                            None
                        };
                        (boid.step(
                            neighbors,
                            self.bounds.0, self.bounds.1,
                            self.flock_scare,
                            target
                        ), returned_neighbors)
                    }
                )
                .collect();
                for (boid, neighbors) in result.iter(){
                    if boid.chosen{
                        if boid.chosen{
                            for neigh in neighbors.as_ref().unwrap(){
                                if (neigh.position - boid.position).abs() < LOCAL_SIZE{
                                    let direction = neigh.velocity.normalized();
                                    canvas
                                        .draw_triangle(
                                            (  
                                                neigh.position.x as i32,
                                                neigh.position.y as i32,
                                            ),
                                            // bottom left: (sin+90 * fac) + world
                                            (
                                                boid.position.x as i32,
                                                boid.position.y as i32,
                                            ),
                                            // bottom right: (sin-90 * fac) + world
                                            (
                                                (boid.position.x as f32) as i32,
                                                (boid.position.y as f32) as i32,
                                            ),
                                        )
                                        .unwrap();
                        
                                }

                            }
                        }
                    }
                }
                let result: Vec<Boidee> = result.into_iter().map(|(boid, _)| boid).collect();
            } else{ // IF VISUALIZE NEIGHBORS FEATURE ISN'T SET ----------------------------------------------------------------
                let result: Vec<Boidee> = c.iterate_flattened().collect::<Vec<&Boidee>>()
                .into_par_iter().map(
                    |boid|{ boid.step(
                            c.get_cell_neighbors(boid),
                            self.bounds.0, self.bounds.1,
                            self.flock_scare,
                            target,
                            self.visual_range
                        )
                    }
                )
                .collect();
            }
        } // END CFG_IF SECTION ------------------------------------------------------------------------------------------------
        
        // flattened Vec over boidees

        *b = Grid::from_vec(result, self.visual_range * 1.0);
        // the buffers have been updated
        //self.dt = Instant::now();
        self.switch = !self.switch;
    }

    /// Initializes `num` [`Boidee`]s with randomized position and velocities. All spawned [`Boidee`]s will
    /// be within the bounds of the Boid they were spawned in
    pub fn init_boidee_random(&mut self, num: usize) {
        let rand = Grid::random(num, self.visual_range, self.bounds.0, self.bounds.1);
        self.b0 = rand.clone();
        self.b1 = rand;
        self.switch = false;

    }
    /// Initilaizes from from a [`Vec`] of [`Boidee`]s. 
    pub fn init_boidees(&mut self, v: Vec<Boidee>) {
        let new = Grid::from_vec(v, self.visual_range);
        self.b0 = new.clone();
        self.b1 = new;
        self.switch = false;
    }
    /// Sets the 'flock scare' of the [`Boidee`]s. This number will be multiplied to the cohesion rule.
    /// A negative value will make them avoid one another.
    /// A positive value will make them move toward one another.
    /// A zero value will disable the cohesion rule altogether.
    /// This value being set to `None` does the same thing as `Some(1.0)`.
    /// Values further from zero will multiply the effect (we're just multiplying a scalar to a vector);
    pub fn set_flock_scare(&mut self, factor: Option<f32>){
        self.flock_scare = factor;
    }

    /// Same as `set_flock_scare`, except returns `self` for nicer chaining.
    pub fn with_flock_scare(self, factor: Option<f32>) -> Self{
        Self{
            flock_scare: factor,
            ..self
        }
    }

    /// Sets whether to draw [`Boidee`]s tiny, and if so how much. Values closer to zero are smaller.
    /// You can also use this function to draw big boidees (their radius of perception will remain the same,
    /// so resizing can make things look strange sometimes).
    /// This value being `None` does the same thing as `Some(1.0)`
    pub fn set_tiny(&mut self, state: Option<f32>){
        self.tiny = state;
    }

    /// Same as `set_tiny`, except returns `self` for nicer chaining.
    pub fn with_tiny(self, state: Option<f32>) -> Self{
        Self{
            tiny: state,
            ..self
        }
    }

    /// Sets the "bounds" of the [`Boidee`]s. When they are within 1/10th of width/height to an edge, they will 
    /// receive a constant acceleration away from that edge.
    /// For example, with bounds (-100, -50), (200, 100), any boid at x coordinates > 170 will receive an acceleration
    ///  of (-1,0) every step. (200 - -100) / 10 = 30, 200 - 30 = 170
    /// Later I'll make the edge percentage customizable, it shouldn't be hard.
    pub fn set_bounds(&mut self, new: ((f32, f32),(f32, f32))){
        self.bounds = new;
    }

    /// Same as `set_bounds`, except returns `self` for nicer chaining.
    pub fn with_bounds(self, new: ((f32, f32),(f32, f32))) -> Self{
        Self{
            bounds: new,
            ..self
        }
    }

    /// Sets the schedule of the [step_on_schedule](Boid::step_on_schedule) function. If it gets done stepping and
    /// drawing boidees before time, it will sleep the remaining time.
    pub fn set_schedule(&mut self, new: Option<Duration>){
        self.schedule = new;
    }

    /// Same as `set_schedule`, excepts returns `self` for nicer chaining.
    pub fn with_schedule(self, new: Option<Duration>) -> Self{
        Self{
            schedule: new,
            ..self
        }
    }

    /// Gets the schedule of the [`Boid`].
    pub fn schedule(&self) -> Option<Duration>{
        self.schedule
    }

    /// Replaces existing [`Boidee`]s with the ones in v.
    pub fn set_boidees(&mut self, v: Vec<Boidee>){
        self.b1 = Grid::from_vec(v, self.visual_range);
        self.switch = true; // this is why you're not allowed direct access fields,
                            // self.b0 currently contains the wrong data
    }

    /// Same as `set_boidees` except returns `self` for nicer chaining
    pub fn with_boidees(self, v: Vec<Boidee>) -> Self{
        Self{
            b1: Grid::from_vec(v, self.visual_range),
            switch: true,
            ..self
        }
    }
    /// Sets how far the [`Boidee`]s can see.
    /// If this value is <= 0 they cannot see
    pub fn with_vision_range(self, new: f32) -> Self{

        Self{
            visual_range: new,
            ..self
        }
    }

    pub fn set_vision_range(&mut self, new: f32){
        self.visual_range = new;
    }

}