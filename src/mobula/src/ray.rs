#![allow(dead_code)]

use crate::point::Point;
use crate::v3::V3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point,
    direction: V3,
}

impl Ray {
    pub fn new(origin: Point, direction: V3) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    pub fn direction(self) -> V3 {
        self.direction
    }

    pub fn origin(self) -> Point {
        self.origin
    }

    pub fn at_parameter(self, t: f64) -> Point {
        self.origin.translate(self.direction * t)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point::origin(),
            direction: V3::zero(),
        }
    }
}
