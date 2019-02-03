use serde::{Serialize, Deserialize};

use std::ops::{Add, Sub};

use crate::v3::V3;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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

impl Add<V3> for Point {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3::from(self) + other
    }
}

impl Sub<V3> for Point {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        V3::from(self) - other
    }
}

impl Sub<Point> for Point {
    type Output = V3;
    fn sub(self, other: Point) -> V3 {
        V3::from(self) - V3::from(other)
    }
}