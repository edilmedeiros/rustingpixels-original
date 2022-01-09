use image::Rgb;
use noise::{NoiseFn, Perlin, Seedable};

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;

    let mut rng = Perlin::new().set_seed(2);

    let colors = [
        Rgb([0xF1, 0xFA, 0xEE]),
        Rgb([0xA8, 0xDA, 0xDC]),
        Rgb([0x45, 0x7B, 0x9D]),
        Rgb([0x1D, 0x35, 0x57]),
        Rgb([0xE6, 0x39, 0x46]),
    ];

    let mut image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    5.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day003a.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    15.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day003b.png").unwrap();

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(
                x,
                y,
                f(
                    x as f64 / width as f64,
                    y as f64 / height as f64,
                    45.0,
                    &mut rng,
                    &colors,
                ),
            );
        }
    }

    image.save("images/day003c.png").unwrap();
}

fn f<R: NoiseFn<[f64; 2]>>(x: f64, y: f64, scale: f64, rng: &mut R, colors: &[Rgb<u8>]) -> Rgb<u8> {
    let index = (rng.get([scale * x, scale * y]) / 2.0 * 9.0).floor() as usize;
    //println!("{}", index);
    colors[index]
}
