use crate::LOCAL_SIZE;
use crate::boid::Boid;
use crate::boidee::Boidee;
use crate::vector2::Vector2;
#[derive(Clone, Debug)]
pub struct Grid{
    max: (f32, f32),
    min: (f32, f32),
    cells: Vec<Vec<Vec<Boidee>>>,
    fac: f32,
}
impl Grid{
    pub fn new(min: (f32,f32), max: (f32, f32), fac: f32) -> Grid{
        // size of vector is max - min, rounded up
        // eg. min:(-11.3, -15.0) max: (110.3, 403.5) will make a grid of (122, 419)
        // this means there will always be a boid in cell [0][0]
        // it also getting something out of the grid involves subtracting min from it
        let cells: Vec<Vec<Vec<Boidee>>> = Grid::init_grid_vec(((max.0 - min.0).ceil() as usize, (max.1 - min.1).ceil() as usize), 1.0/fac);
        Self { max, min, cells, fac: 1.0/fac }
    }
    pub fn from_vec(data: Vec<Boidee>, fac: f32) -> Grid{
        // fac is how large the cells should be, it can be a third of a boids fov (i think)
        // make an array of cells of the right size
        // populate the Vec's of the cells with references to the data
        // profit

        // Vec of Vec's of Vec's of Boidees
        //      0: 0|1|2|3|4
        //      1: 0|1|2|3|4 <-- each of these is a Vec of Boidee's
        //      2: 0|1|2|3|4
        //      3: 0|1|2|3|4
        //      4: 0|1|2|3|4

        let fac = 1.0 / fac; // dividing is slow
        let mut min = (0.0, 0.0);
        let mut max = (0.0, 0.0);
        for boid in data.iter(){
            // dynamically resize
            if boid.position.x > max.0{
                max.0 = boid.position.x;
            }
            if boid.position.y > max.1{
                max.1 = boid.position.y;
            }
            if boid.position.x < min.0{
                min.0 = boid.position.x;
            }
            if boid.position.y < min.1{
                min.1 = boid.position.y;
            }
        }
        // empty 3D array (3rd dimension for Boidees)
        // same as in new(): maximum size for the boids given
        let mut cells: Vec<Vec<Vec<Boidee>>> = Grid::init_grid_vec(((max.0 - min.0).floor() as usize + 1, (max.1 - min.1).floor() as usize + 1), fac);
        // fill them with data
        // this will panic is max is too small, so make sure max isn't too small
        for boidee in data{
            // we subtract min so we get an index > 0
            let index_x = ((boidee.position.x - min.0) * fac).floor() as usize;
            let index_y = ((boidee.position.y - min.1) * fac).floor() as usize;
            cells[index_x][index_y].push(boidee);
        }
        Self { max, min, cells, fac }
    }
    pub fn get_cell_neighbors(&self, sub: &Boidee) -> Vec<Boidee>{
        // we need to subtract min from the postition before using it as an index
        let mut neighbors: Vec<Boidee> = vec![];
        let index_x: usize = ((sub.position.x - self.min.0) * self.fac).floor() as usize;
        let index_y: usize = ((sub.position.y - self.min.1) * self.fac).floor() as usize;
        let x_0 = index_x <= 1;
        let y_0 = index_y <= 1;
        let y_max = index_y >= ((self.max.1 - self.min.1) as f32 * self.fac).floor() as usize;
        let x_max = index_x >= ((self.max.0 - self.min.0) as f32 * self.fac).floor() as usize;
        
        if !x_0{
            // left
            neighbors.append(&mut self.cells[index_x - 1][index_y].clone());
            if !y_0{
                //upper left
                neighbors.append(&mut self.cells[index_x - 1][index_y - 1].clone());
            }
            if !y_max{
                // lower left
                neighbors.append(&mut self.cells[index_x - 1][index_y + 1].clone());
            }
        }
        if !x_max{
            //right
            neighbors.append(&mut self.cells[index_x + 1][index_y].clone());
            if !y_0{
                // upper right
                neighbors.append(&mut self.cells[index_x + 1][index_y - 1].clone());
            }
            if !y_max{
                // lower right
                neighbors.append(&mut self.cells[index_x + 1][index_y + 1].clone());
            }
        }
        if !y_max{
            // down
            neighbors.append(&mut self.cells[index_x][index_y + 1].clone());
        }
        if !y_0{
            // up
            neighbors.append(&mut self.cells[index_x][index_y - 1].clone());
        }
        // we also need our own cell of course
        neighbors.append(&mut self.cells[index_x][index_y].clone());
        // neighbors.append(&mut self.cells[self.cells.len() - 1][self.cells[self.cells.len() - 1].len() - 1].clone());

        neighbors
    }
cfg_if::cfg_if!{
if #[cfg(feature = "visualize_neighbors")]{
    pub fn random(num: usize, min: (f32, f32), max: (f32, f32)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        let mut chosen = Boidee{
            chosen: true,
            ..Boidee::random(min, max)
        };
        v.push(chosen);
        for _ in 1..num {
            v.push(Boidee::random(min, max));
        }
        Grid::from_vec(v, LOCAL_SIZE)

    }
}else{
    pub fn random(num: usize, min: (f32, f32), max: (f32, f32)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..num {
            v.push(Boidee::random(min, max));
        }
        Grid::from_vec(v, LOCAL_SIZE)

    }
}
}
    pub fn init_num(num: u32) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..num{
            v.push(Boidee::new());
        }
        Grid::from_vec(v, LOCAL_SIZE)
    }

    fn init_grid_vec(max: (usize, usize), fac: f32) -> Vec<Vec<Vec<Boidee>>> {
        let mut x_array: Vec<Vec<Vec<Boidee>>> = Vec::new();
        for _ in 0..(((max.0  as f32 * fac)).ceil() as usize){
            let mut y_array: Vec<Vec<Boidee>> = Vec::new();
            for _ in 0..(((max.1 as f32 * fac)).ceil() as usize){
                y_array.push(Vec::new());
            }
            x_array.push(y_array);
        }
        x_array
    }

    pub fn iterate_flattened(&self) -> impl Iterator<Item = &Boidee>{
        self.cells.iter().flatten().flatten()
    }
}

// tests

#[test]
fn is_init_grid_vec_correct_size(){
    let new_grid = Grid::init_grid_vec((8, 6), 1.0/2.0);
    assert_eq!(4_usize, new_grid.len());
    assert_eq!(3_usize, new_grid[3].len());
}

#[test]
fn is_init_grid_vec_working_with_positions(){
    use crate::vector2::Vector2;

    let data = vec![
        Boidee{position: Vector2{x: 16.0, y: 9.0}, .. Boidee::new()},
        Boidee{position: Vector2{x: -16.0, y: -9.0}, .. Boidee::new()},
        
    ];


    let mut min = (0.0, 0.0);
    let mut max = (0.0, 0.0);
    for boid in data.iter(){
        if boid.position.x > max.0{
            max.0 = boid.position.x;
        }
        if boid.position.y > max.1{
            max.1 = boid.position.y;
        }
        if boid.position.x < min.0{
            min.0 = boid.position.x;
        }
        if boid.position.y < min.1{
            min.1 = boid.position.y;
        }
    }
    dbg!((min.1, max.1));
    let max = ((max.0 - min.0).ceil() as usize, (max.1 - min.1).ceil() as usize);
    let new_grid = Grid::init_grid_vec(max, 1.0/2.0);
    assert_eq!(16_usize, new_grid.len());
    assert_eq!(9_usize, new_grid[7].len());
}