use image::Rgb; 

fn main() {
    let width: u32 = 1080;
    let height: u32 = 1080;
    
    let mut image = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x, y, f(x as f64/width as f64, y as f64/height as f64));
        }
    }
    
    image.save("images/day001.png").unwrap();
}

fn f(x: f64, y: f64) -> Rgb<u8> {
    let color1 = Rgb([0xE6, 0x39, 0x46]);
    let color2 = Rgb([0xF1, 0xFA, 0xEE]);
    let color3 = Rgb([0xA8, 0xDA, 0xDC]);
    let color4 = Rgb([0x45, 0x7B, 0x9D]);
    let color5 = Rgb([0x1D, 0x35, 0x57]);

    let square_dist = (x * x + y * y).sqrt();
    
    if square_dist < 0.2 {
        color1
    } else if square_dist < 0.4 {
        color2
    } else if square_dist < 0.6 {
        color3
    } else if square_dist < 0.8 {
        color4
    } else if square_dist < 1.0 {
        color5
    } else {
        color1
    }
}
