use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::v3::V3;

pub trait Hitable {
    fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub intersection: Point,
    pub normal: V3,
    pub material: Material,
    // TODO: Can we maybe have a more descriptive name? (2019-02-03)
    pub t: f64,
}

impl Hit {
    pub fn new(intersection: Point, normal: V3, material: Material, t: f64) -> Self {
        Hit {
            intersection, normal, material, t
        }
    }
}