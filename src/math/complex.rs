use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(real: f64, im: f64) -> Self {
        Self { real, im }
    }

    pub const fn zero() -> Self {
        Self { real: 0.0, im: 0.0 }
    }

    pub fn add_mut(&mut self, other: &Self) {
        self.real += other.real;
        self.im += other.im;
    }

    pub fn square_mut(&mut self) {
        let temp = (self.real * self.real) - (self.im * self.im);
        let im = 2.0 * self.real * self.im;
        let real = temp;

        self.real = real;
        self.im = im;
    }

    pub fn dot(&self) -> f64 {
        (self.real * self.real) + (self.im * self.im)
    }

    pub fn abs(&self) -> f64 {
        self.dot().sqrt()
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            im: self.im - other.im,
        }
    }
}
