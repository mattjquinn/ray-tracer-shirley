use std::ops::{Index, Mul, Add, Div, Sub};

pub struct Vec3 {
    e : [f64; 3],
}

impl Vec3 {
    pub fn new(e0 : f64, e1 : f64, e2 : f64) -> Vec3 {
        Vec3 { e : [e0, e1, e2] }
    }

    pub const fn x(&self) -> f64 { self.e[0] }
    pub const fn y(&self) -> f64 { self.e[1] }
    pub const fn z(&self) -> f64 { self.e[2] }
    pub const fn r(&self) -> f64 { self.e[0] }
    pub const fn g(&self) -> f64 { self.e[1] }
    pub const fn b(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
         self / self.length()
    }

    pub fn dot(v1 : &Vec3, v2 : &Vec3) -> f64 {
        v1[0] * v2[0] + v1[1] * v2[1]  + v1[2] * v2[2]
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}

impl<'a, 'b> Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'a Vec3) -> Vec3 {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<'a, 'b> Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'a Vec3) -> Vec3 {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<'a, 'b> Mul<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Vec3 {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

pub struct Ray<'a> {
    a : &'a Vec3,
    b : &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(a : &'a Vec3, b : &'a Vec3) -> Ray<'a> {
        Ray { a, b }
    }

    pub fn origin(&self) -> &'a Vec3 { self.a }
    pub fn direction(&self) -> &'a Vec3 { self.b }
}