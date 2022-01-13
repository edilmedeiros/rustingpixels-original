use image::{ImageBuffer, Pixel};

#[derive(PartialEq, Debug)]
pub struct Canvas<P: Pixel, Container> {
    pub width: u32,
    pub height: u32,
    pub transform_matrix: TransformMatrix,
    pub buffer: ImageBuffer<P, Container>,
}

pub trait Surface {
    fn dimensions(&self) -> (u32, u32);
    fn matrix(&self) -> &TransformMatrix;
}

impl<P: Pixel, Container> Surface for Canvas<P, Container> {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn matrix(&self) -> &TransformMatrix {
        &self.transform_matrix
    }
}

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
