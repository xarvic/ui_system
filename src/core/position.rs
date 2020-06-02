use std::ops::{Add, Mul, AddAssign, MulAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector{
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Vector{
        Vector{x, y}
    }
    pub const fn null() -> Vector{
        Vector::new(0.0, 0.0)
    }
    pub fn x(self, x: f32) -> Vector {
        Vector{x: self.x + x, y: self.y}
    }
    pub fn y(self, y: f32) -> Vector {
        Vector{x: self.x, y: self.y + y}
    }
    pub fn xy(self, x: f32, y: f32) -> Vector {
        Vector{x: self.x + x, y: self.y + y}
    }
}

impl Into<[f32; 2]> for Vector {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Add for Vector{
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl Mul<f32> for Vector{
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector{
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl<T> Into<Vector> for (T, T) where T: Into<f32> {
    fn into(self) -> Vector {
        Vector::new(self.0.into(), self.1.into())
    }
}