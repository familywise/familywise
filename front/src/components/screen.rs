#[derive(Copy, Clone, PartialEq)]
pub struct ScreenHeight(f64);

impl ScreenHeight {
    pub fn new(height: f64) -> Self {
        Self(height)
    }

    pub fn height(&self) -> f64 {
        self.0
    }
}
