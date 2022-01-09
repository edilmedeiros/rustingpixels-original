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
        self.scale * (self.offset + f64::sin(f64::powi((x - self.x0) * self.period_x, 2) + f64::powi((y - self.y0) * self.period_y, 2))) 
    }
}

