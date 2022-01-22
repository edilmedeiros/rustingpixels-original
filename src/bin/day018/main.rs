use rustingpixels::primitives::{
    canvas::{Canvas, MemoryImage},
    point::*, transform::*,
};
use image::Rgba;

fn main() {

    let width: u32 = 1080;
    let height: u32 = 1080;
    
    let red = Rgba([0x30, 0x00, 0x7f, 0xff]);
    let white = Rgba([0xff, 0xff, 0xff, 0xff]);
    let mut canvas = MemoryImage {width, height, buffer: vec![red; (width * height) as usize]};

    let point: Point2<f64> = Point2::new(50.0, 0.0);

    for scale in 1..10 {
        let transform: Scale2<f64> = Scale2::new(scale as f64, scale as f64);
        let p1 = transform * point;
        for angle in 0..360 {
            let rotation: Rotation2<f64> = Rotation2::new(angle as f64 / 180.0 * std::f64::consts::PI);
            let p2 = rotation * p1;
            canvas.put_pixel(p2.x.floor() as i32, p2.y.floor() as i32, white);
        }
    }
    
    let image = canvas.into_rgba_image();
    image.save("images/day018.png").unwrap();
}
