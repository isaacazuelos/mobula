#![allow(dead_code)]

use mobula::v3::V3;
use mobula::point::Point;

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: V3,
}

impl Ray {
    pub fn new(origin: Point, direction: V3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn direction(self) -> V3 {
        self.direction
    }

    pub fn origin(self) -> Point {
        self.origin
    }

    pub fn at_parameter(self, t: f64) -> Point {
        self.origin.translate(self.direction.scale(t))
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(Point::default(), V3::default())
    }
}
