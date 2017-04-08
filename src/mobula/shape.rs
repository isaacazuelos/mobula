use mobula::ray::Ray;
use mobula::v3::V3;

pub trait Shape {
    fn is_hit_by(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    pub centre: V3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: V3, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Shape for Sphere {
    fn is_hit_by(&self, ray: &Ray) -> bool {
        let oc = ray.origin().v3() - self.centre;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
