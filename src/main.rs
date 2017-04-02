extern crate image;

mod mobula;

use std::path::Path;

use image::ImageBuffer;

use mobula::v3::V3;
use mobula::point::Point;
use mobula::ray::Ray;

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
    // In the sample code, this would be followed by a `.normalize()` but this leads to the
    // resulting image being different than the one in the book.
    let direction = ray.direction();
    let t = 0.5 * (direction.y() + 1.0);
    let start = V3::new(1.0, 1.0, 1.0);
    let end = V3::new(0.5, 0.7, 1.0);
    linear_interpolation(start, end, t)
}

fn main() {
    let lower_left_corner = V3::new(-2.0, -1.0, -1.0);
    let horizontal = V3::new(4.0, 0.0, 0.0);
    let vertical = V3::new(0.0, 2.0, 0.0);
    let origin = Point::origin();

    let img = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let j = IMAGE_HEIGHT - y; // count down
        let i = x;

        let u: f64 = (i as f64) / (IMAGE_WIDTH as f64);
        let v: f64 = (j as f64) / (IMAGE_HEIGHT as f64);

        let r = Ray::new(origin,
                         lower_left_corner + horizontal.scale(u) + vertical.scale(v));
        let c = colour(r);
        println!("{:?}", c);

        let ir = (255.99 * c.x()) as u8;
        let ig = (255.99 * c.y()) as u8;
        let ib = (255.99 * c.z()) as u8;


        let p = image::Rgb([ir, ig, ib]);
        p
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
}
