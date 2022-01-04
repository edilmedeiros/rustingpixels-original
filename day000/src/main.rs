use image::Rgb; 

fn main() {
    let width: u32 = 500;
    let height: u32 = 500;
    let bg = Rgb([0x45, 0x7b, 0x9d]);
    
    let image = image::RgbImage::from_pixel(width, height, bg);
    image.save("day000.png").unwrap();
}
