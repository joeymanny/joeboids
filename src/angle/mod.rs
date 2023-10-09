use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Angle (f32);
impl Angle {
    pub fn new (mut x: f32) -> Angle {
        x  %= 2.0 * PI;
        if x < 0.0{
            Angle((2.0 * PI) + x)
        }else{
            Angle(x)
        }
    }
    pub fn face(self, wish: Angle) -> f32 {
        let x = wish.0 - self.0;
        if x.abs() <= PI{
            x
        }else {
            x - (2.0 * PI) 
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
impl Add for Angle {
    type Output = Self;
    fn add<>(self, other: Angle) -> Self::Output {
            Self((self.0 + other.0) % (2.0 * PI))
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, other: Angle) -> Self{
        let mut ans = self.0 - other.0;
        ans %= PI * 2.0;
        if ans < 0.0 {
            Self(ans + 2.0 * PI)
        }else{
            Self(ans)
        }
    }
}
impl Div<f32> for Angle{
    type Output = Angle;
    fn div(self, rhs: f32) -> Self::Output {
        if self.0 > PI{
            Angle((2.0 * PI) - ((2.0 * PI - self.0) / rhs))
        }else{
            Angle(self.0 / rhs)
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
impl Add<Angle> for f32{
    type Output = Angle;
    fn add(self, rhs: Angle) -> Self::Output {
        Angle((rhs.0 + self) % (2.0 * PI))
    }
}
impl Sub<Angle> for f32{
    type Output = Angle;
    fn sub(self, rhs: Angle) -> Self::Output {
        let rtrn = (rhs.0 - self) % (2.0 * PI);
        if rtrn < 0.0{
            Angle((2.0 * PI) - rtrn)
        }else {
            Angle(rtrn)
        }
    }
}