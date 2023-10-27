use crate::LOCAL_SIZE;
use crate::boidee::Boidee;
#[derive(Clone, Debug)]
pub struct Grid{
    max: (usize, usize),
    cells: Vec<Vec<Vec<Boidee>>>,
    fac: f32,
}
impl Grid{
    pub fn new(max: (usize,usize), fac: f32) -> Grid{
        let cells: Vec<Vec<Vec<Boidee>>> = Grid::init_grid_vec(max, fac);
        Self { max, cells, fac }
    }
    pub fn from_vec(data: Vec<Boidee>, max: (usize, usize), fac: f32) -> Grid{ // DONE!
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
        let mut buf: Vec<Vec<Vec<Boidee>>> = Grid::init_grid_vec((max.0, max.1), fac);
        // fill them with data
        // this will panic is max is too small, so make sure max isn't too small
        for boidee in data{
            let index_x = (boidee.position.x / fac).floor() as usize;
            let index_y = (boidee.position.y / fac).floor() as usize;
            buf[index_x][index_y].push(boidee);
        }
        Self { max, cells: buf, fac }
    }
    pub fn get_cell_neighbors(&self, sub: &Boidee) -> Vec<Boidee>{
        // we are assuming that all Boidees have positions within the max
        // we can assume this because these (should be) both coordinated by Boid
        // just don't mess up Boid and it'll be fine
        let mut rtrn: Vec<Boidee> = vec![];
        let index_x: usize = (sub.position.x / self.fac).floor() as usize;
        let index_y: usize = (sub.position.y / self.fac).floor() as usize;
        let sub_cell = self.cells[index_x][index_y].clone();
        let x_0 = index_x <= 1;
        let y_0 = index_y <= 1;
        let y_max = index_y >= ((self.max.1) as f32 / self.fac).floor() as usize;
        let x_max = index_x >= ((self.max.0) as f32 / self.fac).floor() as usize;
        
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
            rtrn.append(&mut self.cells[index_x - 1][index_y].clone());
            if !y_0{
                //upper left
                rtrn.append(&mut self.cells[index_x - 1][index_y - 1].clone());
            }
            if !y_max{
                // lower left
                rtrn.append(&mut self.cells[index_x - 1][index_y + 1].clone());
            }
        }else{
            // rtrn.append(&mut self.cells[self.cells.len() - 1][y_adj].clone());
        }
        if !x_max{
            //right
            rtrn.append(&mut self.cells[index_x + 1][index_y].clone());
            if !y_0{
                // upper right
                rtrn.append(&mut self.cells[index_x + 1][index_y - 1].clone());
            }
            if !y_max{
                // lower right
                rtrn.append(&mut self.cells[index_x + 1][index_y + 1].clone());
            }
        }
        if !y_max{
            // down
            rtrn.append(&mut self.cells[index_x][index_y + 1].clone());
        }
        if !y_0{
            // up
            rtrn.append(&mut self.cells[index_x][index_y - 1].clone());
        }
        // we also need our own cell of course
        rtrn.append(&mut sub_cell.clone());
        rtrn.append(&mut self.cells[self.cells.len() - 1][self.cells[self.cells.len() - 1].len() - 1].clone());

        rtrn
    }
    pub fn random(num: usize, bounds: (usize, usize)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..(num
        ){
            v.push(Boidee::random(bounds));
        }
        Grid::from_vec(v, bounds, LOCAL_SIZE)

    }
    pub fn init_num(num: u32, bounds: (usize, usize)) -> Grid{
        let mut v:Vec<Boidee> = vec![];
        for _ in 0..num{
            v.push(Boidee::new());
        }
        Grid::from_vec(v, bounds, LOCAL_SIZE)
    }

    fn init_grid_vec(max: (usize, usize), fac: f32) -> Vec<Vec<Vec<Boidee>>> {
        let mut x_array: Vec<Vec<Vec<Boidee>>> = Vec::new();
        for _ in 0..(((max.0 as f32 / fac) + 1.0).ceil() as usize){
            let mut y_array: Vec<Vec<Boidee>> = Vec::new();
            for _ in 0..(((max.1 as f32 / fac) + 1.0).ceil() as usize){
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
