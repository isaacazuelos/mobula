use mobula::hit::{Hitable, Hit};
use mobula::ray::Ray;

pub type World = Vec<Box<Hitable>>;

pub fn nearest_hit(world: &World, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut hit = None;
    let mut closest_so_far = t_max;
    for obj in world {
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
