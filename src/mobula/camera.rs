use mobula::point::Point;
use mobula::ray::Ray;
use mobula::v3::V3;

pub struct Camera {
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
    origin: Point,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_left_corner: V3::new(-2.0, -1.0, -1.0),
            horizontal: V3::new(4.0, 0.0, 0.0),
            vertical: V3::new(0.0, 2.0, 0.0),
            origin: Point::origin(),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v) -
                 self.origin.v3())
    }
}
