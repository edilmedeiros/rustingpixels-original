use image::Rgb;
//use noise::{NoiseFn, Perlin, Seedable};
use rand::prelude::*;
use scarlet::{colormap::*, prelude::*};

use day006::Ripple;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
    let mut image = image::RgbImage::new(width, height);

    // good candidates = 2, 4, 5
    let mut rng = StdRng::seed_from_u64(5);
    //    let mut rng = Perlin::new().set_seed(2);

    let colors = ListedColorMap::plasma();

    let ripple1 = Ripple {
        x0: rng.gen(),
        y0: rng.gen(),
        period_x: rng.gen::<f64>() * 5.0,
        period_y: rng.gen::<f64>() * 5.0,
        scale: rng.gen(),
        offset: rng.gen(),
    };

    let ripple2 = Ripple {
        x0: rng.gen(),
        y0: rng.gen(),
        period_x: rng.gen::<f64>() * 5.0,
        period_y: rng.gen::<f64>() * 5.0,
        scale: rng.gen(),
        offset: rng.gen(),
    };

    let ripple3 = Ripple {
        x0: rng.gen(),
        y0: rng.gen(),
        period_x: rng.gen::<f64>() * 5.0,
        period_y: rng.gen::<f64>() * 5.0,
        scale: rng.gen(),
        offset: rng.gen(),
    };

    println!("{:?}", ripple1);
    println!("{:?}", ripple2);
    println!("{:?}", ripple3);

    for x in 0..width {
        for y in 0..height {
            let z = ripple1.z(x as f64 / width as f64, y as f64 / height as f64);
            let output: RGBColor = colors.transform_single(z);
            image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()]));
        }
    }

    image.save("images/day006a.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            let z = ripple2.z(x as f64 / width as f64, y as f64 / height as f64);
            let output: RGBColor = colors.transform_single(z);
            image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()]));
        }
    }

    image.save("images/day006b.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            let z = ripple3.z(x as f64 / width as f64, y as f64 / height as f64);
            let output: RGBColor = colors.transform_single(z);
            image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()]));
        }
    }

    image.save("images/day006c.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            let z1 = ripple1.z(x as f64 / width as f64, y as f64 / height as f64);
            let z2 = ripple2.z(x as f64 / width as f64, y as f64 / height as f64);
            let z3 = ripple3.z(x as f64 / width as f64, y as f64 / height as f64);
            let output: RGBColor = colors.transform_single(z1 + z2 + z3);
            image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()]));
        }
    }

    image.save("images/day006d.png").unwrap();
}
