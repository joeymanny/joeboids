use std::ops::{Div,Mul, Sub, Add};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl From<Vector2> for (i32, i32){
    fn from(val: Vector2) -> Self{
        (val.x.floor() as i32, val.y.floor() as i32)
    }
}
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    pub fn abs(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn normalized(self) -> Self {
        let fac = 1.0 / self.abs();
        Vector2 {
            x: self.x * fac,
            y: self.y * fac,
        }
    }
    pub fn zero() -> Vector2{
        Vector2 { x: 0.0, y: 0.0 }
    }
    pub fn left() -> Self{
        Self {
            x: -1.0,
            y: 0.0
        }
    }
    pub fn right() -> Self{
        Self {
            x: 1.0,
            y: 0.0
        }
    }
    pub fn up() -> Self{
        Self {
            x: 0.0,
            y: 1.0
        }
    }
    pub fn down() -> Self{
        Self {
            x: 0.0,
            y: -1.0
        }
    }
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
impl Add<f32> for Vector2{
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output{
        Vector2{
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}
impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
