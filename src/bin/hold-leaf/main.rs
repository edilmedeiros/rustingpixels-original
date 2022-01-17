use image::{Pixel, Rgba};
use rand::prelude::*;
use std::f64;

use rustingpixels::primitives::canvas::*;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;

    let bg = Rgba([0x26, 0x46, 0x53, 0xFF]);
    let color = Rgba([0xe9, 0xc4, 0x6a, 0x20]);

    let mut image = image::RgbaImage::from_pixel(width, height, bg);
    
    let matrix = TransformMatrix {
        xx: (width - 1) as f64 / 8.0,
        yx: 0.0,
        xy: 0.0,
        yy: (height - 1) as f64 / 11.0,
        x0: width as f64 / 2.0,
        y0: height as f64 / 1080.0,
    };

    let mut p = Point { x: 0.0, y: 0.0 };
    let mut rng = StdRng::seed_from_u64(1);

    let f1: fn(Point<f64>) -> Point<f64> = |p| {affine_transformation(0.0, 0.0, 0.0, 0.16, 0.0, 0.0, p)};
    let f2: fn(Point<f64>) -> Point<f64> = |p| {affine_transformation(0.85, 0.04, -0.04, 0.85, 0.0, 1.6, p)};
    let f3: fn(Point<f64>) -> Point<f64> = |p| {affine_transformation(0.2, -0.26, 0.23, 0.22, 0.0, 1.6, p)};
    let f4: fn(Point<f64>) -> Point<f64> = |p| {affine_transformation(-0.15, 0.28, 0.26, 0.24, 0.0, 0.44, p)};

    const ITERATIONS: u32 = 3_000_000;
    for _i in 0..ITERATIONS {
        let (x, y) = p.point_to_canvas_coordinate(&matrix);
        if point_is_visible(x, y, image.width(), image.height()) {
            image.get_pixel_mut(x, y).blend(&color);
        }
        p = barnsley_fern(p, f1, f2, f3, f4, &mut rng);
    }
    image
        .save("images/day013-0.png")
        .unwrap();

}

fn barnsley_fern<R, F>(p: Point<f64>, f1: F, f2: F, f3: F, f4: F, rng: &mut R) -> Point<f64>
where
    R: Rng,
    F: Fn (Point<f64>) -> Point<f64> {
    match rng.gen::<f64>() {
        r if r <= 0.01 => f1(p),
        r if r <= 0.86 => f2(p),
        r if r <= 0.93 => f3(p),
        _                   => f4(p),
    }
}

fn affine_transformation(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, p: Point<f64>) -> Point<f64> {
    Point {
        x: a * p.x + b * p.y + e,
        y: c * p.x + d * p.y + f,
    }
}

fn point_is_visible(x: u32, y: u32, width: u32, height: u32) -> bool {
    if x > 0 && x < width && y > 0 && y < height {
        true
    } else {
        false
    }
}
