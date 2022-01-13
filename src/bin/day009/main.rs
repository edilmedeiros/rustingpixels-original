use image::Rgb;
//use rand::prelude::*;
use scarlet::{colormap::*, prelude::*};

use rustingpixels::Crazy;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
    let mut image = image::RgbImage::new(width, height);

    // good candidates = 5
    //let mut rng = StdRng::seed_from_u64(5);

    let colors = vec![ListedColorMap::plasma()];

    let crazy =  
        Crazy {
            x0: 0.35,
            y0: 0.35,
            scale_x: 3.0,
            scale_y: 3.0,
            scale: 0.2,
            ..Crazy::default()
        };

    for (count, color) in colors.into_iter().enumerate() {
        for x in 0..width {
            for y in 0..height {
                let z = crazy.z(x as f64 / width as f64, y as f64 / height as f64);
                let output: RGBColor = color.transform_single(z);

                let eps = 2e-3;
                match z {
                    k if f64::abs(k - 1.6) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.5) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.4) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.3) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.2) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.1) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 1.0) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.9) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.8) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.7) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.6) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.5) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.4) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.3) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.2) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.1) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    k if f64::abs(k - 0.0) < eps =>
                        image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()])),
                    _ => ()
                }
            }
        }

        image
            .save("images/day009-".to_owned() + &count.to_string() + ".png")
            .unwrap();
    }
}
