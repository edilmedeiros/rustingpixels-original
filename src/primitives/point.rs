extern crate nalgebra as na;
pub use nalgebra::geometry::Point2;

//pub type Point<T> = Point2<T>;


//pub type Vector<T> = Vector3<T>;



//use crate::primitives::transform::TransformMatrix;

// #[derive(PartialEq, Debug)]
// pub struct Point<T> {
//     pub x: T,
//     pub y: T,
// }

// impl Point<f64> {
//     pub fn point_to_canvas_coordinate(&self, matrix: &TransformMatrix) -> (u32, u32) {
//         let x_new: u32 = (self.x * matrix.xx + self.y * matrix.xy + matrix.x0).floor() as u32;
//         let y_new: u32 = (self.x * matrix.yx + self.y * matrix.yy + matrix.y0).floor() as u32;
//         (x_new, y_new)
//     }

    // pub fn raster<C: Surface>(&self, canvas: C) -> Vec<Point<u32>> {
    //     let matrix = canvas.matrix();
    //     let x_canvas: u32 = (self.x * matrix.xx + self.y * matrix.xy + matrix.x0).floor() as u32;
    //     let y_canvas: u32 = (self.x * matrix.yx + self.y * matrix.yy + matrix.y0).floor() as u32;
    //     vec![Point {
    //         x: x_canvas,
    //         y: y_canvas,
    //     }]
    // }
//}

// impl Iterator for Point {
//     type Item = (u32, u32);
//     fn next(&mut self) -> Option<(u32, u32)>
// }
