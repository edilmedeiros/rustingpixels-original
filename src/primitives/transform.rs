extern crate nalgebra as na;

pub use nalgebra::geometry::{Translation2, Scale2, Rotation2};


#[derive(PartialEq, Debug)]
pub struct TransformMatrix {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}

impl TransformMatrix {
    fn new() -> Self {
        TransformMatrix::default()
    }

    // fn translate(&self, Point<f64>) {
        
    // }
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




