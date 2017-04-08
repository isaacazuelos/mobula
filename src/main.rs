extern crate image;

mod mobula;

use std::path::Path;

use image::ImageBuffer;

use mobula::v3::V3;
use mobula::ray::Ray;
use mobula::point::Point;
use mobula::shape::{Shape, Sphere};

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;

fn assert_clamped(value: f64) {
    assert!(value >= 0.0, "expected value >= zero, found: {}", value);
    assert!(value <= 1.0, "expected value >= one, found {}", value);
}

fn linear_interpolation(start: V3, end: V3, t: f64) -> V3 {
    assert_clamped(t);
    start.scale(1.0 - t) + end.scale(t)
}

fn colour(ray: Ray) -> V3 {
    let s = Sphere::new(V3::new(0.0, 0.0, -1.0), 0.5);
    if s.is_hit_by(&ray) {
        return V3::new(1.0, 0.0, 0.0);
    }
    let direction = ray.direction().normalize();
    let t = 0.5 * (direction.y() + 1.0);
    let start = V3::new(1.0, 1.0, 1.0);
    let end = V3::new(0.5, 0.7, 1.0);
    linear_interpolation(start, end, t)
}

fn main() {
    let lower_left_corner = Point::new(-2.0, -1.0, -1.0);
    let horizontal = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 2.0, 0.0);
    let origin = Point::origin();

    let img = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |i, j| {
        let ray = {
            let u = (i as f64) / (IMAGE_WIDTH as f64);
            let v = (j as f64) / (IMAGE_HEIGHT as f64);
            let horz = horizontal.v3().scale(u);
            let vert = vertical.v3().scale(v);

            Ray::new(origin, lower_left_corner.v3() + horz + vert)
        };

        let colour = colour(ray);

        let ir = (255.99 * colour.x()) as u8;
        let ig = (255.99 * colour.y()) as u8;
        let ib = (255.99 * colour.z()) as u8;

        println!("colour for {:?} is 0x{:X}{:X}{:X}", ray, ir, ig, ib);

        let p = image::Rgb([ir, ig, ib]);
        p
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
}
