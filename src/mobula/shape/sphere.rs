use crate::mobula::hit::{Hit, Hitable};
use crate::mobula::material::Material;
use crate::mobula::point::Point;
use crate::mobula::ray::Ray;
use crate::mobula::v3::V3;

#[derive(Copy, Clone, Debug)]
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
    fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // first, we use the quadratic formula to solve for the roots (number of intersections).
        let oc = V3::from(ray.origin()) - V3::from(self.centre);
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None; // There are no collisions.
        }

        let mut hit = Hit::new();
        hit.material = self.material;

        // first we'll try the - branch of the +- in the quadratic equation.
        let temp = (-b - (b * b - a * c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            hit.t = temp;
            hit.intersection = ray.at_parameter(hit.t);
            hit.normal = (V3::from(hit.intersection) - V3::from(self.centre))
                .scale(1.0 / self.radius)
                .normalize();
            return Some(hit);
        }

        // now the + branch
        let temp = (-b + (b * b - a * c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            hit.t = temp;
            hit.intersection = ray.at_parameter(hit.t);
            hit.normal = (V3::from(hit.intersection) - V3::from(self.centre))
                .scale(1.0 / self.radius)
                .normalize();
            return Some(hit);
        }

        None
    }
}
