use rand::random;
use serde::{Deserialize, Serialize};

use crate::config::Config;
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CameraBuilder {
    origin: Point,
    target: Point,
    up: V3,
    fov: f64,
    aspect: Option<f64>, // None means compute from config
    aperture: f64,
    focus_distance: f64,
}

impl Default for CameraBuilder {
    fn default() -> CameraBuilder {
        let origin = Point::origin();
        let target = Point::new(0.0, 0.0, 1.0);
        CameraBuilder {
            origin,
            target,
            up: V3::new(0.0, 1.0, 0.0),
            fov: 20.0,
            aspect: None,
            aperture: 2.0,
            focus_distance: (origin - target).magnitude(),
        }
    }
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        Self::default()
    }

    pub fn build(self, config: &Config) -> Camera {
        let theta = self.fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * self.aspect.unwrap_or(f64::from(config.width) / f64::from(config.height));

        let w = (self.origin - self.target).normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lens_radius: self.aperture / 2.0,
            origin: self.origin,
            u,
            v,
            horizontal: u.scale(2.0 * half_width * self.focus_distance),
            vertical: v.scale(2.0 * half_height * self.focus_distance),
            lower_left_corner: Point::from(
                self.origin
                    - u.scale(half_width * self.focus_distance)
                    - v.scale(half_height * self.focus_distance)
                    - w.scale(self.focus_distance),
            ),
        }
    }

    pub fn origin(mut self, point: Point) -> Self {
        self.origin = point;
        self
    }

    pub fn target(mut self, point: Point) -> Self {
        self.target = point;
        self
    }

    pub fn up(mut self, direction: V3) -> Self {
        self.up = direction;
        self
    }

    pub fn fov(mut self, value: f64) -> Self {
        self.fov = value;
        self
    }

    pub fn aspect(mut self, ratio: f64) -> Self {
        self.aspect = Some(ratio);
        self
    }

    pub fn aperture(mut self, value: f64) -> Self {
        self.aperture = value;
        self
    }

    pub fn focus_distance(mut self, value: f64) -> Self {
        self.focus_distance = value;
        self
    }
}

#[derive(Clone, Copy, Debug)]
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
