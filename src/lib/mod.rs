use std::ops::Index;

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
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}
