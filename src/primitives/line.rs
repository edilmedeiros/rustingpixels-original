pub mod line {

    #[derive(Debug)]
    pub struct Line {
        pub x0: f64,
        pub y0: f64,
        pub x1: f64,
        pub y1: f64
    }

    impl Default for Line {
        fn default() -> Self {
            Line {
                x0: 0.0,
                y0: 0.0,
                x1: 0.0,
                y1: 0.0
            }
        }
    }

}
