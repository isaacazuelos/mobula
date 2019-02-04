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
    pub material: Material,
    pub normal: V3,
    // TODO: Can we maybe have a more descriptive name? (2019-02-03)
    pub t: f64,
}

impl Default for Hit {
    fn default() -> Hit {
        Hit {
            intersection: Point::origin(),
            material: Material::default(),
            normal: V3::default(),
            t: 0.0,
        }
    }
}