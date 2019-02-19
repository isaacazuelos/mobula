use serde::{Deserialize, Serialize};

use crate::colour::Colour;
use crate::hit::Hit;
use crate::material::Scatter;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(self, _ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        let target = hit.intersection + hit.normal + Scatter::random_in_unit_sphere();

        Some((
            self.albedo,
            Ray::new(hit.intersection, target - hit.intersection.into()),
        ))
    }
}
