use std::ops::{Add, Sub};

use super::TextureCoord;
use color::Color;

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    im: f64,
}

impl Complex {
    const fn new(real: f64, im: f64) -> Self {
        Self { real, im }
    }

    fn zero() -> Self {
        Self { real: 0.0, im: 0.0 }
    }

    fn add_mut(&mut self, other: &Self) {
        self.real += other.real;
        self.im += other.im;
    }

    fn square_mut(&mut self) {
        let temp = (self.real * self.real) - (self.im * self.im);
        let im = 2.0 * self.real * self.im;
        let real = temp;

        self.real = real;
        self.im = im;
    }

    fn dot(&self) -> f64 {
        (self.real * self.real) + (self.im * self.im)
    }

    fn abs(&self) -> f64 {
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

const MAX: Complex = Complex::new(1.0, 1.2);
const MIN: Complex = Complex::new(-2.1, -1.2);
const MAX_ITERATIONS: usize = 500;

pub fn mandelbrot(coord: TextureCoord) -> Color {
    let mut z = Complex::default();
    let c = Complex::new(
        MIN.real + (coord.x as f64 % 1.0) * (MAX.real - MIN.real),
        MIN.im + (1.0 - (coord.y as f64 % 1.0)) * (MAX.im - MIN.im),
    );

    for i in 0..MAX_ITERATIONS {
        z.square_mut();
        z.add_mut(&c);

        if z.dot() > 4.0 {
            let smooth_i = i as f64 + 1.0 - (z.abs().log2() / 2.0_f64.log2()).log2();

            let hue = 320.0 + 360.0 * (smooth_i as f32 / MAX_ITERATIONS as f32);
            let saturation = 1.0;
            let value = 1.0;

            return Color::new_hsv(hue, saturation, value);
        }
    }

    Color::black()
}
