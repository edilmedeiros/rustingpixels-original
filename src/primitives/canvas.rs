//use core::ops::{Deref, DerefMut};
//use std::path::Path;
use image::{Rgba, RgbaImage};

//pub type RgbaCanvas = Canvas<Rgba<u8>, Vec<u8>>;

pub trait Canvas {
    type P: image::Pixel;

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn put_pixel(&mut self, x: i32, y: i32, pixel: Self::P);
    fn get_pixel(&self, x: i32, y: i32) -> Option<&Self::P>;
    
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    
    fn is_inside_canvas(&self, x: i32, y: i32) -> bool {
        let (sx, sy) = self.to_screen_coordinates(x, y);
        sx < self.width() && sy < self.height()
    }

    fn to_screen_coordinates(&self, x: i32, y: i32) -> (u32, u32) {
        let sx: u32 = ((self.width() / 2) as i32 + x) as u32;
        let sy: u32 = ((self.height() / 2) as i32 - y) as u32;
        (sx, sy)
    }


}

pub struct MemoryImage {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Rgba<u8>>,
}

impl MemoryImage {
    pub fn into_vec(self) -> Vec<Rgba<u8>> {
        self.buffer
    }

    pub fn into_rgba_image(self) -> RgbaImage {
        RgbaImage::from_fn(self.width, self.height, |x, y| {
            *self
                .get_pixel(
                    x as i32 - self.width as i32 / 2,
                    self.width as i32 / 2 - y as i32,
                )
                .unwrap()
        })
    }

    fn coordinates_to_index(&self, x: i32, y: i32) -> Option<usize> {
        let (sx, sy) = self.to_screen_coordinates(x, y);
        if self.is_inside_canvas(x, y) {
            Some(sx as usize + self.width as usize * sy as usize)
        } else {
            None
        }
    }
}

impl Canvas for MemoryImage {
    type P = Rgba<u8>;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn put_pixel(&mut self, x: i32, y: i32, pixel: Self::P) {
        if let Some(index) = self.coordinates_to_index(x, y) {
            self.buffer[index] = pixel
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Option<&Self::P> {
        let index = self.coordinates_to_index(x, y)?;
        Some(&self.buffer[index])
    }
}

impl Canvas for RgbaImage {
    type P = Rgba<u8>;

    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }

    fn dimensions(&self) -> (u32, u32) {
        self.dimensions()
    }

    fn put_pixel(&mut self, x: i32, y: i32, pixel: Self::P) {
        let sx: u32 = ((self.width() / 2) as i32 + x) as u32;
        let sy: u32 = ((self.height() / 2) as i32 - y) as u32;
        if sx < self.width() && sy < self.height() {
            self.put_pixel(sx, sy, pixel)
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Option<&Self::P> {
        let sx: u32 = ((self.width() / 2) as i32 + x) as u32;
        let sy: u32 = ((self.height() / 2) as i32 - y) as u32;
        if sx < self.width() && sy < self.height() {
            Some(self.get_pixel(sx, sy))
        } else {
            None
        }
    }
}

// #[derive(PartialEq, Debug)]
// pub struct Canvas<P, Container>
// where
//     P: Pixel + 'static,
//     P::Subpixel: 'static,
//     Container: Deref<Target = [P::Subpixel]> + DerefMut,
// {
//     width: u32,
//     height: u32,
// //    transform_matrix: TransformMatrix,
//     buffer: ImageBuffer<P, Container>,
// }

// impl<P: Pixel, Container> Canvas<P, Container>
// where
//     P: Pixel + 'static,
//     P::Subpixel: 'static,
//     Container: Deref<Target = [P::Subpixel]> + DerefMut,
// {
//     pub fn put_pixel(&mut self, x: i32, y: i32, pixel: P) {
//         let sx: u32 = (self.width as i32 / 2 + x) as u32;
//         let sy: u32 = (self.height as i32 / 2 - y) as u32;

//         // Check if the given coordinates are visible.
//         // sx and sy are unsigned, only check upper bounds.
//         if sx < self.width && sy < self.height {
//             self.buffer.put_pixel(sx, sy, pixel);
//         }
//     }
// }

// impl<P: Pixel + 'static> Canvas<P, Vec<P::Subpixel>>
// where
//     P::Subpixel: 'static
// {
//     pub fn new(width: u32, height: u32) -> Canvas<P, Vec<P::Subpixel>> {
//         Canvas {
//             width,
//             height,
//             buffer: ImageBuffer::new(width, height),
//         }
//     }

//     pub fn from_pixel(width: u32, height: u32, pixel: P) -> Canvas<P, Vec<P::Subpixel>> {
//         Canvas {
//             width,
//             height,
//             buffer: ImageBuffer::from_pixel(width, height, pixel),
//         }
//     }

//     pub fn from_fn<F>(width: u32, height: u32, f: F) -> Canvas<P, Vec<P::Subpixel>>
//     where
//         F: FnMut(u32, u32) -> P,
//     {
//         Canvas {
//             width,
//             height,
//             buffer: ImageBuffer::from_fn(width, height, f),
//         }
//     }
// }

// impl<P, Container> Canvas<P, Container>
// where
//     P: Pixel + 'static,
//     [P::Subpixel]: EncodableLayout,
//     Container: Deref<Target = [P::Subpixel]> + DerefMut,
// {
//     pub fn save<Q>(&self, path: Q) -> ImageResult<()>
//     where
//         Q: AsRef<Path>,
//     {
//         self.buffer.save(path)
//     }
// }

// pub trait Surface {
//     fn dimensions(&self) -> (i32, i32);
//     fn matrix(&self) -> &TransformMatrix;
// }

// impl<P: Pixel> Surface for Canvas<P> {
//     fn dimensions(&self) -> (i32, i32) {
//         (self.width, self.height)
//     }

//     fn matrix(&self) -> &TransformMatrix {
//         &self.transform_matrix
//     }
// }
