use image::{Pixel, Rgba};
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use rustingpixels::primitives::canvas::*;

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
//    let black = Rgba([0, 0, 0, 255]);

    let palette1 = vec![
        Rgba([0xE6, 0x39, 0x46, 255]),
        Rgba([0xF1, 0xFA, 0xEE, 255]),
        Rgba([0xA8, 0xDA, 0xDC, 255]),
        Rgba([0x45, 0x7B, 0x9D, 255]),
        Rgba([0x1D, 0x35, 0x57, 255]),
    ];

    let palette2 = vec![
        Rgba([0x00, 0x64, 0x66, 255]),
        Rgba([0x06, 0x5A, 0x60, 255]),
        Rgba([0x0B, 0x52, 0x5B, 255]),
        Rgba([0x14, 0x45, 0x52, 255]),
        Rgba([0x1B, 0x3A, 0x4B, 255]),
        Rgba([0x21, 0x2f, 0x45, 255]),
        Rgba([0x27, 0x26, 0x40, 255]),
        Rgba([0x31, 0x22, 0x44, 255]),
        Rgba([0x3E, 0x1F, 0x47, 255]),
        Rgba([0x4D, 0x19, 0x4D, 255]),
    ];

    let palette3 = vec![
        Rgba([0x7B, 0x11, 0x12, 255]),
        Rgba([0x8C, 0x1F, 0x27, 255]),
        Rgba([0xFF, 0xB3, 0x02, 255]),
        Rgba([0xDB, 0x91, 0x01, 255]),
        Rgba([0x6E, 0x09, 0x0C, 255]),
    ];

//    let palettes = vec![palette1, palette2, palette3];
    let mut rng = StdRng::seed_from_u64(5);

    let bg = vec![&palette3[4], &palette1[4], &palette1[4], &palette2[6]];
    let color = vec![&palette3[2], &palette1[0], &palette1[1], &palette2[0]];

    
    for k in 0..4 {
        let mut image = image::RgbaImage::from_pixel(width, height, *bg[k]);

        let mut matrices = Vec::new();
        let i_bound = 20;
        let j_bound = 20;
        for i in 0..i_bound {
            for j in 0..j_bound {
                let spacing = 20.0;
                let w = ((width - 1) as f64 - (i_bound + 1) as f64 * spacing) as f64 / i_bound as f64;
                let h = ((height - 1) as f64 - (j_bound + 1) as f64 * spacing) as f64 / j_bound as f64;
                matrices.push(TransformMatrix {
                    xx: w,
                    yx: 1.0,//((i % 2_u32) as f64).signum() * i_bound as f64,
                    xy: 1.0,//((j % 2_u32) as f64).signum() * j_bound as f64,
                    yy: h,
                    x0: spacing + (i as f64 * (w + spacing)),
                    y0: spacing + (j as f64 * (h + spacing)),
                });
            }
        }

        //let palette = &palettes[rng.gen_range(0, palettes.len())];
        for matrix in matrices.iter() {
            //let alpha: u8 = rng.gen_range(150, 255);
            //let mut color = palette[rng.gen_range(0, palette.len())];
            //color.channels_mut()[3] = alpha;
                        
            let density: u32 = rng.gen_range(50..1000);
            for _j in 0..density {
                let p = random_point(&mut rng);
                let (x, y) = p.point_to_canvas_coordinate(matrix);

                //println!("{:?}", p);
                //println!("{:?}", (x,y));

                if point_is_visible(x, y, image.width(), image.height()) {
                    image.get_pixel_mut(x, y).blend(color[k]);
                    //image.put_pixel(x, y, *color);
                }
            }
        }
        image.save(format!("images/day014-{}.png", k)).unwrap();
    }
}

fn point_is_visible(x: u32, y: u32, width: u32, height: u32) -> bool {
    if x > 0 && x < width && y > 0 && y < height {
        true
    } else {
        false
    }
}

// fn scaled_random_point<R: Rng>(rng: &mut R, x_scale: f64, y_scale: f64) -> Point<f64> {
//     Point {
//         x: (x_scale * rng.gen::<f64>() + 1.0).ln() / f64::ln(x_scale + 1.0),
//         y: (y_scale * rng.gen::<f64>() + 1.0).ln() / f64::ln(y_scale + 1.0),
//     }
// }

fn random_point<R>(rng: &mut R) -> Point<f64>
where R: Rng {
    let normal = Normal::new(0.5, 0.3).unwrap();
    Point {
        x: normal.sample(rng),
        y: normal.sample(rng),
    }
}
