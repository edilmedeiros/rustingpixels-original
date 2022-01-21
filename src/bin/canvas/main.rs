use rustingpixels::primitives::canvas::{Canvas, RgbaCanvas};
use image::Rgba;

fn main() {

    let red = Rgba([0x7f, 0x00, 0x00, 0xff]);
    let black = Rgba([0,0,0,0xff]);
    let mut canvas: RgbaCanvas = Canvas::from_pixel(100, 100, red);
     

    for x in 0..100 {
        canvas.put_pixel(x, 0, black);
    }
    canvas.save("images/test.png").unwrap();
}
