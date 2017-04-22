use mobula::v3::V3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    /// A zero vector.
    pub fn origin() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }

    /// Move a point by a vector
    pub fn translate(self, v: V3) -> Point {
        Point::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    /// Construct a new point.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    // Turn a point into a vector, which is used a lot for math.
    pub fn v3(self) -> V3 {
        V3::new(self.x, self.y, self.z)
    }
}
