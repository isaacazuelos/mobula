use rand::random;

use mobula::point::Point;
use mobula::ray::Ray;
use mobula::v3::V3;

use std::f64::consts::PI;

fn random_in_unit_disk() -> V3 {
    let mut p = V3::new(10.0, 0.0, 0.0);
    while p.magnitude() >= 1.0 {
        p = (V3::new(random(), random(), 0.0) - V3::new(1.0, 1.0, 0.0)).scale(2.0);
    }
    p
}

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point,
               lookat: Point,
               vup: V3,
               fov: f64,
               aspect: f64,
               aperture: f64,
               focus_distance: f64)
               -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;

        let w = (lookfrom.v3() - lookat.v3()).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            lens_radius: aperture / 2.0,
            origin: lookfrom,
            u: u,
            v: v,
            horizontal: u.scale(2.0 * half_width * focus_distance),
            vertical: v.scale(2.0 * half_height * focus_distance),
            lower_left_corner: Point::from_v3(lookfrom.v3() - u.scale(half_width * focus_distance) -
                                              v.scale(half_height * focus_distance) -
                                              w.scale(focus_distance)),
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x) + self.v.scale(rd.y);
        Ray::new(self.origin.translate(offset),
                 self.lower_left_corner.v3() + self.horizontal.scale(s) +
                 self.vertical.scale(t) - self.origin.v3() - offset)
    }
}
