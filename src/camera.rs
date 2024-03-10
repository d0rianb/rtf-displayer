pub struct Camera {
    pub offset_x: f32,
    pub offset_y: f32,
    pub lock_x: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            lock_x: true, // by default, we do not authorize horizontal scroll
        }
    }
}

impl Camera {
    pub fn scroll(&mut self, x: f64, y: f64) {
        if !self.lock_x {
            self.offset_x = f32::min(0., self.offset_x + x as f32);
        }
        self.offset_y = f32::min(0., self.offset_y + y as f32);
    }

    pub fn reset(&mut self) {
        self.offset_x = 0.;
        self.offset_y = 0.;
    }
}