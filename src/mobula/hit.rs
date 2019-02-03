use crate::mobula::material::Material;
use crate::mobula::point::Point;
use crate::mobula::ray::Ray;
use crate::mobula::v3::V3;

pub struct Hit {
    pub t: f64,
    pub intersection: Point,
    pub normal: V3,
    pub material: Material,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            t: 0.0,
            intersection: Point::origin(),
            normal: V3::zero(),
            material: Material::default(),
        }
    }
}

pub trait Hitable {
    fn is_hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
