use serde::{Deserialize, Serialize};

use crate::hit::{Hit, Hitable};
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub centre: Point,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64, material: Material) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    // TODO: we should really have a better name than t_min and t_max
    fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // first, we use the quadratic formula to solve for the roots (number of
        // intersections).
        let oc = ray.origin() - self.centre;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        // t1 and t2 are the two solutions to our quadratic
        let t1 = (-b - discriminant.sqrt()) / a;
        let t2 = (-b + discriminant.sqrt()) / a;

        if discriminant < 0.0 {
            None // There are no collisions.
        } else if t1 < t_max && t1 > t_min {
            let intersection = ray.at_parameter(t1);
            let normal = ((intersection - self.centre) * (1.0 / self.radius)).normalize();
            Some(Hit::new(intersection, normal, self.material, t1))
        } else if t2 < t_max && t2 > t_min {
            let intersection = ray.at_parameter(t2);
            let normal = ((intersection - self.centre) * (1.0 / self.radius)).normalize();
            Some(Hit::new(intersection, normal, self.material, t2))
        } else {
            None
        }
    }
}
