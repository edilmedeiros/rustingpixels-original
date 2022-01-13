use image::{Rgba, Pixel};
use rand::prelude::*;
//use scarlet::{colormap::*, prelude::*};

//use rustingpixels::Crazy;

use rustingpixels::primitives::canvas::*;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
    let black = Rgba([0,0,0,255]);

    let palette = vec![
        Rgba([0xE6, 0x39, 0x46, 255]),
        Rgba([0xF1, 0xFA, 0xEE, 255]),
        Rgba([0xA8, 0xDA, 0xDC, 255]), 
        Rgba([0x45, 0x7B, 0x9D, 255]),
        Rgba([0x1D, 0x35, 0x57, 255])
    ];

    
    let mut image1 = image::RgbaImage::from_pixel(width, height, black);
    let mut image2 = image::RgbaImage::from_pixel(width, height, black);

    // Good candidates: 5
    let mut rng = StdRng::seed_from_u64(4);

    let mut matrices = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            matrices.push(
                TransformMatrix {
                    xx: width as f64 / 10.0,
                    yx: 0.0,
                    xy: 0.0,
                    yy: height as f64 / 10.0,
                    x0: width as f64 / 10.0 * i as f64,
                    y0: height as f64 / 10.0 * j as f64,
                }
            );
        }
    }

    for matrix in matrices.iter() {
        let mut gray = Rgba([255,255,255,255]);
        gray.channels_mut()[3] = rng.gen();

        let mut color = palette[rng.gen_range(0,5)];
        color.channels_mut()[3] = rng.gen();
        
        let density: u32 = rng.gen_range(500, 5000);
        for _j in 0..density {
            let p = random_point(&mut rng);
            let (x, y) = p.point_to_canvas_coordinate(matrix);
            image1.put_pixel(x, y, gray);
            image2.put_pixel(x, y, color);
        }
    }
    
    image1.save("images/day010-0.png").unwrap();
    image2.save("images/day010-1.png").unwrap();

    
}

fn random_point<R: Rng>(rng: &mut R) -> Point<f64> {
    Point {
        x: rng.gen(),
        y: rng.gen(),
    }
}
