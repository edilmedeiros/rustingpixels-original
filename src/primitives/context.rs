use crate::primitives::{canvas, transform};

pub struct Context<C: canvas::Canvas> {
    canvas: C,
    //transform: transform::Transform<T>,
}


