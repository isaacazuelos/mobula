use crate::mobula::hit::{Hit, Hitable};
use crate::mobula::ray::Ray;

pub struct World {
    objects: Vec<Box<Hitable>>,
}

impl World {
    pub fn new() -> Self {
        World { objects: Vec::new() }
    }

    pub fn nearest_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            match obj.is_hit_by(ray, t_min, closest_so_far) {
                None => {}
                Some(nearer_hit) => {
                    closest_so_far = nearer_hit.t;
                    hit = Some(nearer_hit);
                }
            }
        }
        hit      
    }

    pub fn add(&mut self, object: Box<Hitable>) {
        self.objects.push(object);
    }
}
