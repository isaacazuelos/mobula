use serde::{Serialize, Deserialize};

use crate::hit::{Hit, Hitable};
use crate::ray::Ray;

pub mod sphere;

// This would be nicer as a trait, but the generic bounds for serde prevent
// making a trait object of shapes.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(tag="type")]
pub enum Shape {
    Sphere(sphere::Sphere),
}

impl Hitable for Shape {
    fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Shape::Sphere(s) => s.is_hit_by(ray, t_min, t_max)
        }
    }
}