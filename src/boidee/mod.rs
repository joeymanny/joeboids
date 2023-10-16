const RAND_BOID_SPEED: f32 = 4.0;
const RAND_BOID_SPEED_VARIATION: f32 = 0.5;
use crate::vector2::Vector2;
use crate::angle::Angle;
use crate::{MAX_RAND_SCOPE, TOO_CLOSE, LOCAL_SIZE};
use rand::prelude::*;
use std::f32::consts::PI;
use crate::grid::Grid;
#[derive(Debug, Clone, PartialEq)]
pub struct Boidee {
    pub pos: Vector2,
    pub dir: Angle,
    pub speed: f32,
    pub randscope: usize,
    pub rand: f32,
    pub chosen: bool
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
    pub fn random(bounds: (usize, usize)) -> Boidee {
        let mut r = rand::thread_rng();
        Boidee {
            pos: Vector2::new(
                r.gen::<f32>() * bounds.0 as f32,
                r.gen::<f32>() * bounds.1 as f32,
            ),
            dir: Angle::new(r.gen::<f32>() * (PI * 2.0)),
            speed: RAND_BOID_SPEED - ((r.gen::<f32>() - 0.5) * (RAND_BOID_SPEED_VARIATION * 2.0)),
            randscope: 0,
            rand: 0.0,
            chosen: false
        }
    }
    pub fn new() -> Boidee {
        Boidee {
            pos: Vector2::new(0.0, 0.0),
            dir: Angle::new(0.0),
            speed: 2.0,
            randscope: 0,
            rand: 0.0,
            chosen: false,
        }
    }
    pub fn step(
        &self,
        flock: &Grid,
        bounds: &(usize, usize),
        flock_scare: Option<f32>
    ) -> 
        (
            Boidee
            ,Option<Vec<Boidee>>)
        {
        let mut new_dir = self.dir;
        let mut move_by = Vector2::new(0.0, 0.0);
        let mut local_avg = Vector2::new(0.0, 0.0);
        let mut local_num = 0;
        let mut local_dir = self.dir;
        let mut too_close_p = Vector2::new(0.0, 0.0);
        let mut too_close_n = 0;
        let mut amogus: Option<Vec<Boidee>> = None;
        let neighbors = flock.get_cell_neighbors(&self);
        let mut used_speed = self.speed;
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
                    local_dir =  ((fren.dir - local_dir) / 2.0) + local_dir;
                    local_avg = local_avg + fren.pos;
                    local_num += 1;
                    if self.chosen {
                        amogus.as_mut().unwrap().push(fren);
                    }
                }
            }
        }
        // all adjustments that rely on local group averages
        if local_num != 0 {
            if too_close_n != 0 {
                too_close_p = too_close_p / too_close_n as f32;
                // avoid locals too close
                move_by = move_by + ((too_close_p) - self.pos) / -10.0;
            }
            local_avg = local_avg / local_num as f32;
            if let Some(f) = flock_scare{
                // go towards center of local cluster
                move_by = move_by + ((local_avg - self.pos) / 80.0 * f);
                used_speed = (f.abs() / 20.0) * used_speed + 6.0; // flock_scare affects this a bit
            }else{
                move_by = move_by + ((local_avg - self.pos) / 80.0);

                // try face local average
                new_dir = new_dir + Angle::new(self.dir.face(local_dir) / 7.0);
            }
        }
        let mut r = rand::thread_rng();

        let new_randscope;
        let new_rand;
        if self.randscope <= 0 {
            new_randscope = (r.gen::<f32>() * MAX_RAND_SCOPE as f32) as usize;
            new_rand = ((r.gen::<f32>() - 0.5) / 8.0) / ((50.0 + local_num as f32) / 50.0);
        } else {
            new_dir = new_dir + self.rand;
            new_randscope = self.randscope - 1;
            new_rand = self.rand;
        }
        // boid steps forward
        move_by =
            move_by + self.pos + Vector2::new(new_dir.cos() * used_speed, new_dir.sin() * used_speed);

        // all modifications to pos & dir should be done before this point
        move_by.x = move_by.x % bounds.0 as f32;
        move_by.y = move_by.y % bounds.1 as f32;
        if move_by.x < 0.0 {
            move_by.x += bounds.0 as f32;
        }
        if move_by.y < 0.0 {
            move_by.y += bounds.1 as f32;
        }
        new_dir = new_dir % (2.0 * PI);
        if new_dir < 0.0 {
            new_dir = new_dir + (2.0 * PI) ;
        }
        (
            Boidee {
            pos: move_by,
            dir: new_dir,
            speed: self.speed,
            randscope: new_randscope,
            rand: new_rand,
            chosen: self.chosen
        }
        , amogus)
    }
}
