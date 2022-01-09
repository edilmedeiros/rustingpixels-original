use image::Rgb;
use rand::prelude::*;
use scarlet::{colormap::*, prelude::*};

use rustingpixels::Saddle;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
    let mut image = image::RgbImage::new(width, height);

    // good candidates = 5
    let mut rng = StdRng::seed_from_u64(5);

    let colors = ListedColorMap::plasma();

    let saddles: Vec<Saddle> = vec![
        Saddle {
            x0: 0.5,
            y0: 0.5,
            scale_x: 1.5,
            scale_y: 1.5,
            ..Saddle::default()
        },
        Saddle {
            x0: rng.gen(),
            y0: rng.gen(),
            scale_x: rng.gen::<f64>() * 1.0,
            scale_y: rng.gen::<f64>() * 1.0,
            offset: rng.gen::<f64>(),
        },
        Saddle {
            x0: rng.gen(),
            y0: rng.gen(),
            scale_x: rng.gen::<f64>() * 1.0,
            scale_y: rng.gen::<f64>() * 1.0,
            offset: rng.gen::<f64>(),
        },
    ];

    for (count,saddle) in saddles.into_iter().enumerate() {
        for x in 0..width {
            for y in 0..height {
                let z = saddle.z(x as f64 / width as f64, y as f64 / height as f64);
                let output: RGBColor = colors.transform_single(z);
                image.put_pixel(x, y, Rgb([output.int_r(), output.int_g(), output.int_b()]));
            }
        }
        image
            .save("images/day007-".to_owned() + &count.to_string() + ".png")
            .unwrap();
    }
}
