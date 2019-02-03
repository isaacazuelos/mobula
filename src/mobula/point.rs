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

    // Make a point out of a vector.
    pub fn from_v3(v: V3) -> Self {
        Point::new(v.x, v.y, v.z)
    }

    // Turn a point into a vector, which is used a lot for math.
    pub fn to_v3(self) -> V3 {
        V3::new(self.x, self.y, self.z)
    }
}
