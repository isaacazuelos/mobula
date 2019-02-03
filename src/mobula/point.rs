use crate::mobula::v3::V3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    /// Construct a new point.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// A zero vector.
    pub fn origin() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }

    /// Move a point by a vector
    pub fn translate(self, v: V3) -> Point {
        Point::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}


impl From<V3> for Point {
    fn from(v: V3) -> Self {
        unsafe { ::std::mem::transmute(v) }
    }
}