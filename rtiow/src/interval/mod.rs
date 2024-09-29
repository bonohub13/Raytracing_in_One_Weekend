use crate::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: INFINITY,
        max: -INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: -INFINITY,
        max: INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        (self.min <= x) && (x <= self.max)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        (self.min < x) && (x < self.max)
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if self.max < x {
            self.max
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
