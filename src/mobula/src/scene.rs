use serde::{Deserialize, Serialize};

use crate::camera::CameraBuilder;
use crate::config::Config;
use crate::hit::{Hit, Hitable};
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::v3::V3;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub config: Config,
    pub camera: CameraBuilder,
    pub objects: Vec<Box<Shape>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            config: Config::default(),
            camera: CameraBuilder::default(),
            objects: Vec::new(),
        }
    }

    pub fn camera(mut self, camera: CameraBuilder) -> Self {
        self.camera = camera;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn add(mut self, object: Box<Shape>) -> Self {
        self.objects.push(object);
        self
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
}
