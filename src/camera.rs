use rand::random;

use crate::point::Point;
use crate::ray::Ray;
use crate::v3::V3;

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
    pub fn new(
        origin: Point,
        look_at: Point,
        vup: V3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;

        let w = (origin - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            lens_radius: aperture / 2.0,
            origin,
            u,
            v,
            horizontal: u.scale(2.0 * half_width * focus_distance),
            vertical: v.scale(2.0 * half_height * focus_distance),
            lower_left_corner: Point::from(
                origin
                    - u.scale(half_width * focus_distance)
                    - v.scale(half_height * focus_distance)
                    - w.scale(focus_distance),
            ),
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x) + self.v.scale(rd.y);
        Ray::new(
            self.origin.translate(offset),
            self.lower_left_corner + self.horizontal.scale(s) + self.vertical.scale(t)
                - V3::from(self.origin)
                - offset,
        )
    }
}
