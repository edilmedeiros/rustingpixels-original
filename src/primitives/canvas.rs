use core::ops::{Deref, DerefMut};
use std::path::Path;
use image::{ImageBuffer, Pixel, ImageResult, EncodableLayout, Rgba};

pub type RgbaCanvas = Canvas<Rgba<u8>, Vec<u8>>;

#[derive(PartialEq, Debug)]
pub struct Canvas<P, Container>
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    width: u32,
    height: u32,
//    transform_matrix: TransformMatrix,
    buffer: ImageBuffer<P, Container>,
}

impl<P: Pixel, Container> Canvas<P, Container>
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    pub fn put_pixel(&mut self, x: i32, y: i32, pixel: P) {
        let sx: u32 = (self.width as i32 / 2 + x) as u32;
        let sy: u32 = (self.height as i32 / 2 - y) as u32;

        // Check if the given coordinates are visible.
        // sx and sy are unsigned, only check upper bounds.        
        if sx < self.width && sy < self.height {
            self.buffer.put_pixel(sx, sy, pixel);
        }
    }
}

impl<P: Pixel + 'static> Canvas<P, Vec<P::Subpixel>>
where
    P::Subpixel: 'static
{
    pub fn new(width: u32, height: u32) -> Canvas<P, Vec<P::Subpixel>> {
        Canvas {
            width,
            height,
            buffer: ImageBuffer::new(width, height),
        }
    }

    pub fn from_pixel(width: u32, height: u32, pixel: P) -> Canvas<P, Vec<P::Subpixel>> {
        Canvas {
            width,
            height,
            buffer: ImageBuffer::from_pixel(width, height, pixel),
        }
    }

    pub fn from_fn<F>(width: u32, height: u32, f: F) -> Canvas<P, Vec<P::Subpixel>>
    where
        F: FnMut(u32, u32) -> P,
    {
        Canvas {
            width,
            height,
            buffer: ImageBuffer::from_fn(width, height, f),
        }
    }
}

impl<P, Container> Canvas<P, Container>
where
    P: Pixel + 'static,
    [P::Subpixel]: EncodableLayout,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    pub fn save<Q>(&self, path: Q) -> ImageResult<()>
    where
        Q: AsRef<Path>,
    {
        self.buffer.save(path)
    }
}

pub trait Surface {
    fn dimensions(&self) -> (i32, i32);
    fn matrix(&self) -> &TransformMatrix;
}

// impl<P: Pixel> Surface for Canvas<P> {
//     fn dimensions(&self) -> (i32, i32) {
//         (self.width, self.height)
//     }

//     fn matrix(&self) -> &TransformMatrix {
//         &self.transform_matrix
//     }
// }


#[derive(PartialEq, Debug)]
pub struct TransformMatrix {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}

impl Default for TransformMatrix {
    fn default() -> Self {
        TransformMatrix {
            xx: 1.0,
            yx: 0.0,
            xy: 0.0,
            yy: 1.0,
            x0: 0.0,
            y0: 0.0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f64> {
    pub fn point_to_canvas_coordinate(&self, matrix: &TransformMatrix) -> (u32, u32) {
        let x_new: u32 = (self.x * matrix.xx + self.y * matrix.xy + matrix.x0).floor() as u32;
        let y_new: u32 = (self.x * matrix.yx + self.y * matrix.yy + matrix.y0).floor() as u32;
        (x_new, y_new)
    }

    pub fn raster<C: Surface>(&self, canvas: C) -> Vec<Point<u32>> {
        let matrix = canvas.matrix();
        let x_canvas: u32 = (self.x * matrix.xx + self.y * matrix.xy + matrix.x0).floor() as u32;
        let y_canvas: u32 = (self.x * matrix.yx + self.y * matrix.yy + matrix.y0).floor() as u32;
        vec![Point {
            x: x_canvas,
            y: y_canvas,
        }]
    }
}

// impl Iterator for Point {
//     type Item = (u32, u32);
//     fn next(&mut self) -> Option<(u32, u32)>
// }
