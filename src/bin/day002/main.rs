use image::Rgb;
use rand::prelude::*;
//use rand_pcg::Pcg64;


fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;

    let mut rng = StdRng::seed_from_u64(2);

    let colors =
        [
            Rgb([0xE6, 0x39, 0x46]),
            Rgb([0xF1, 0xFA, 0xEE]),
            Rgb([0xA8, 0xDA, 0xDC]),
            Rgb([0x45, 0x7B, 0x9D]),
            Rgb([0x1D, 0x35, 0x57]),
        ];

    
    let mut image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x, y,
                            f(&mut rng, &colors));
        }
    }
    
    image.save("images/day002.png").unwrap();
}

fn f<R: Rng>(rng: &mut R, colors: &[Rgb<u8>]) -> Rgb<u8> {
    colors[rng.gen_range(0,5)]
}
