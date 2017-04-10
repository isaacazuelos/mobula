extern crate image;

mod mobula;

use std::path::Path;

use image::ImageBuffer;

use mobula::point::Point;
use mobula::ray::Ray;
use mobula::shape::sphere::Sphere;
use mobula::v3::V3;
use mobula::world::{World, nearest_hit};

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 200;

fn linear_interpolation(start: V3, end: V3, t: f64) -> V3 {
    start.scale(1.0 - t) + end.scale(t)
}

fn colour(ray: Ray, world: &World) -> V3 {
    match nearest_hit(world, &ray, 0.0, std::f64::MAX) {
        None => {
            let unit = ray.direction().normalize();
            let t = 0.5 * unit.y() + 1.0;
            linear_interpolation(V3::new(1.0, 1.0, 1.0), V3::new(0.5, 0.7, 1.0), t)
        }
        Some(hit) => {
            V3::new(hit.normal.x() + 1.0,
                    hit.normal.y() + 1.0,
                    hit.normal.z() + 1.0)
                    .scale(0.5)
        }
    }
}

fn main() {
    let lower_left_corner = Point::new(-2.0, -1.0, -1.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 2.0, 0.0);
    let origin = Point::origin();

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point::new(0.0, -100.1, -1.0), 100.0)));
    world.push(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));

    let img = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |i, j| {

        let ray = {
            let u = (i as f64) / (IMAGE_WIDTH as f64);
            let v = ((IMAGE_HEIGHT - j) as f64) / (IMAGE_HEIGHT as f64);
            let horz = horizontal.v3().scale(u);
            let vert = vertical.v3().scale(v);

            Ray::new(origin, lower_left_corner.v3() + horz + vert)
        };

        let colour = colour(ray, &world);
        let ir = (255.99 * colour.x()) as u8;
        let ig = (255.99 * colour.y()) as u8;
        let ib = (255.99 * colour.z()) as u8;
        image::Rgb([ir, ig, ib])
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
}
