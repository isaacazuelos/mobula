use serde::{Deserialize, Serialize};

use crate::colour::Colour;
use crate::hit::Hit;
use crate::material::Scatter;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(fuzz: f64, albedo: Colour) -> Metal {
        Metal { fuzz, albedo }
    }
}

impl Scatter for Metal {
    fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        let reflected = ray.direction().normalize().reflect(hit.normal);
        let scattered = Ray::new(
            hit.intersection,
            reflected + (<dyn Scatter>::random_in_unit_sphere() * self.fuzz),
        );
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
