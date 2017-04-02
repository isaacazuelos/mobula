// for now...
#![allow(dead_code)]

use mobula::v3::V3;

#[derive(Copy, Clone, Debug)]
pub struct Point(f64, f64, f64);

impl Point {
    /// The `x` component of the vector.
    pub fn x(&self) -> f64 {
        self.0
    }
    /// The `y` component of the vector.
    pub fn y(&self) -> f64 {
        self.1
    }
    /// The `z` component of the vector.
    pub fn z(&self) -> f64 {
        self.2
    }

    /// A zero vector.
    pub fn origin() -> Self {
        Point(0.0, 0.0, 0.0)
    }

    /// Move a point by a vector
    pub fn translate(self, v: V3) -> Point {
        Point(self.0 + v.x(), self.1 + v.y(), self.2 + v.z())
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::origin()
    }
}
