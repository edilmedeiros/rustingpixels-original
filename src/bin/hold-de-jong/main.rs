use image::{Pixel, Rgba};
use rand::prelude::*;
use std::f64;

use rustingpixels::primitives::canvas::*;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;

    let gray = Rgba([255, 255, 255, 20]);
    let black = Rgba([0, 0, 0, 255]);

    let mut image1 = image::RgbaImage::from_pixel(width, height, black);
    let mut image2 = image::RgbaImage::from_pixel(width, height, black);
    let image3 = image::RgbaImage::from_pixel(width, height, black);

    let matrix = TransformMatrix {
        xx: (width - 1) as f64 / 4.0,
        yx: 0.0,
        xy: 0.0,
        yy: (height - 1) as f64 / 4.0,
        x0: width as f64 / 2.0,
        y0: height as f64 / 2.0,
    };
      

    let mut p = Point { x: 0.0, y: 0.0 };
    let a = 0.97;
    let b = -1.90;
    let c = 1.38;
    let d = -1.5;
    for _i in 0..12_000_000 {
        let (x, y) = p.point_to_canvas_coordinate(&matrix);
        image1.get_pixel_mut(x, y).blend(&gray);
        p = de_jong_ifs(a, b, c, d, p);
    }
    image1.save("images/day011-0.png").unwrap();

    let mut p = Point { x: 0.0, y: 0.0 };
    let a = 1.641;
    let b = 1.902;
    let c = 0.316;
    let d = 1.525;
    for _i in 0..12_000_000 {
        let (x, y) = p.point_to_canvas_coordinate(&matrix);
        image2.get_pixel_mut(x, y).blend(&gray);
        p = de_jong_ifs(a, b, c, d, p);
    }
    image2.save("images/day011-1.png").unwrap();

    let mut p = Point { x: 0.0, y: 0.0 };
    let a = 1.4;
    let b = -2.3;
    let c = 2.4;
    let d = -2.1;
    for _i in 0..12_000_000 {
        let (x, y) = p.point_to_canvas_coordinate(&matrix);
        image2.get_pixel_mut(x, y).blend(&gray);
        p = de_jong_ifs(a, b, c, d, p);
    }
    image3.save("images/day011-2.png").unwrap();

    
    {
        // Good candidates: 5, 6, 8, 15, 17, 19, 20, 52, 56, 58, 62, 66, 69, 73, 75, 77, 83, 86, 92, 95, 
        // 115, 116, 124, 126, 130, 139, 140, 145, 146, 150, 152, 153, 169, 178, 189, 191, 192, 194, 200,
        // 210, 223, 228, 248, 257, 261, 282, 286, 295, 301*, 307, 316, 318, 319*, 329, 351, 362, 363*,
        // 370, 374*, 379, 380, 384, 388, 393, 399, 410, 435, 439*, 456, 481*, 488, 500, 502, 514, 522,
        // 524*, 527, 528, 532, 533, 540, 542, 553, 557, 571, 580, 581, 618, 620*, 622, 634, 663, 669,
        // 672, 681, 684, 687, 688, 690, 691, 697, 713, 720, 726, 728, 755, 762, 763*, 766, 783, 813,
        // 815, 826, 835, 839, 840, 844, 846, 848, 850, 873, 886, 890*, 911, 931, 934, 939, 941, 942,
        // 947, 951, 952, 957, 960, 974, 985, 990, 995, 996, 997
        crossbeam::scope(|spawner| {
            let v = vec![
                5, 6, 8, 15, 17, 19, 20, 52, 56, 58, 62, 66, 69, 73, 75, 77, 83, 86, 92, 95, 
                115, 116, 124, 126, 130, 139, 140, 145, 146, 150, 152, 153, 169, 178, 189, 191,
                192, 194, 200, 210, 223, 228, 248, 257, 261, 282, 286, 295, 301, 307, 316, 318,
                319, 329, 351, 362, 363, 370, 374, 379, 380, 384, 388, 393, 399, 410, 435, 439,
                456, 481, 488, 500, 502, 514, 522, 524, 527, 528, 532, 533, 540, 542, 553, 557,
                571, 580, 581, 618, 620, 622, 634, 663, 669, 672, 681, 684, 687, 688, 690, 691,
                697, 713, 720, 726, 728, 755, 762, 763, 766, 783, 813, 815, 826, 835, 839, 840,
                844, 846, 848, 850, 873, 886, 890, 911, 931, 934, 939, 941, 942, 947, 951, 952,
                957, 960, 974, 985, 990, 995, 996, 997
            ];
            for i in v {
            //for i in 0..1000 {
                spawner.spawn(move |_| {
                    let mut image = image::RgbaImage::from_pixel(width, height, black);
                    let matrix2 = TransformMatrix {
                        xx: (width - 1) as f64 / 4.0,
                        yx: 0.0,
                        xy: 0.0,
                        yy: (height - 1) as f64 / 4.0,
                        x0: width as f64 / 2.0,
                        y0: height as f64 / 2.0,
                    };
                    let mut p = Point { x: 0.0, y: 0.0 };
                    let mut rng = StdRng::seed_from_u64(i);
                    let a: f64 = 2.0 * rng.gen::<f64>();
                    let b: f64 = 2.0 * rng.gen::<f64>();
                    let c: f64 = 2.0 * rng.gen::<f64>();
                    let d: f64 = 2.0 * rng.gen::<f64>();
                    for _i in 0..12_000_000 {
                        let (x, y) = p.point_to_canvas_coordinate(&matrix2);
                        image.get_pixel_mut(x, y).blend(&gray);
                        p = de_jong_ifs(a, b, c, d, p);
                    }
                    image
                        .save(format!("images/de-jong/day011-{}.png", i))
                        .unwrap();
                });
            }
        })
        .unwrap();
    }
}

fn de_jong_ifs(a: f64, b: f64, c: f64, d: f64, p: Point<f64>) -> Point<f64> {
    Point {
        x: (a * p.y).sin() - (b * p.x).cos(),
        y: (c * p.x).sin() - (d * p.y).cos(),
    }
}
