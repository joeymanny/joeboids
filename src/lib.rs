trait BoidCanvas {
    fn draw_triangle(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> Result<(), String>;
    fn new_boid() -> Boid {
        Boid {
            canvas: &mut self,
            b0: Vec<Boidee>::new(),
            b1: Vec<Boidee>::new(),
            switch: false,
        }
    }
}
struct Boid<T: BoidCanvas> {
    canvas: &mut T,
    b0: Vec<Boidee>,
    b1: Vec<Boidee>,
    switch: Bool,
}
impl Boid {
    step(&mut self) {
        if self.switch {
            self.b1.update_boids(&mut self.b0);
            switch = !switch;
        } else {
            self.b0.update_boids(&mut self.b1);
            switch = !switch;
        }
    }
}
impl Vec<Boidee> {
    fn update_boids(&) {
        
    }
}
