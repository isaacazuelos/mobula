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
        p = (V3::new(random(), random(), 0.0) - V3::new(1.0, 1.0, 0.0)) * 2.0;
    }
    p
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CameraBuilder {
    origin: Point,
    target: Point,
    up: V3,
    fov: f64,
    #[serde(default)]
    aperture: f64,
    #[serde(default)]
    aspect: Option<f64>, // None means compute from config
    #[serde(default)]
    focus_distance: Option<f64>, // None means focus on target
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
            focus_distance: None,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        Self::default()
    }

    fn auto_focus_distance(&self) -> f64 {
        match self.focus_distance {
            None => (self.origin - self.target).magnitude(),
            Some(f) => f,
        }
    }

    pub fn build(self, config: &Config) -> Camera {
        let theta = self.fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height
            * self
                .aspect
                .unwrap_or(f64::from(config.width) / f64::from(config.height));

        let w = (self.origin - self.target).normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lens_radius: self.aperture / 2.0,
            origin: self.origin,
            u,
            v,
            horizontal: u * (2.0 * half_width * self.auto_focus_distance()),
            vertical: v * (2.0 * half_height * self.auto_focus_distance()),
            lower_left_corner: Point::from(
                self.origin
                    - (u * (half_width * self.auto_focus_distance()))
                    - (v * (half_height * self.auto_focus_distance()))
                    - (w * (self.auto_focus_distance())),
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
        self.focus_distance = Some(value);
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
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = (self.u * rd.x) + (self.v * rd.y);
        Ray::new(
            self.origin.translate(offset),
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - V3::from(self.origin)
                - offset,
        )
    }
}
