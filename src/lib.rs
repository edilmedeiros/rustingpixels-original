pub mod primitives;


#[derive(Debug)]
pub struct Ripple {
    pub x0: f64,
    pub y0: f64,
    pub period_x: f64,
    pub period_y: f64,
    pub scale: f64,
    pub offset: f64,
}

impl Default for Ripple {
    fn default() -> Self {
        Ripple {
            x0: 0.0,
            y0: 0.0,
            period_x: 1.0,
            period_y: 1.0,
            scale: 1.0,
            offset: 0.0,
        }
    }
}

impl Ripple {
    pub fn z(&self, x: f64, y: f64) -> f64 {
        self.scale
            * (self.offset
                + f64::sin(
                    f64::powi((x - self.x0) * self.period_x, 2)
                        + f64::powi((y - self.y0) * self.period_y, 2),
                ))
    }
}

#[derive(Debug)]
pub struct Saddle {
    pub x0: f64,
    pub y0: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub offset: f64,
}

impl Default for Saddle {
    fn default() -> Self {
        Saddle {
            x0: 0.0,
            y0: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            offset: 0.5,
        }
    }
}

impl Saddle {
    pub fn z(&self, x: f64, y: f64) -> f64 {
        self.offset + f64::powi(self.scale_x * (x - self.x0), 2)
            - f64::powi(self.scale_y * (y - self.y0), 2)
    }
}

#[derive(Debug)]
pub struct Crazy {
    pub x0: f64,
    pub y0: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale: f64,
    pub offset: f64,
}

impl Default for Crazy {
    fn default() -> Self {
        Crazy {
            x0: 0.0,
            y0: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale: 1.0,
            offset: 0.5,
        }
    }
}

impl Crazy {
    pub fn z(&self, x: f64, y: f64) -> f64 {
        self.scale
            * (self.offset
                + f64::powi(self.scale_x * (x - self.x0), 3)
                + f64::powi(self.scale_y * (y - self.y0), 3)
                - 3.0 * self.scale_x * (x - self.x0) * self.scale_y * (y - self.y0))
    }
}
