use super::TextureCoord;
use crate::color::Color;
use crate::math::Complex;

const MAX: Complex = Complex::new(1.0, 1.2);
const MIN: Complex = Complex::new(-2.1, -1.2);
const MAX_ITERATIONS: usize = 130;

pub fn mandelbrot(coord: TextureCoord) -> Color {
    let mut z = Complex::default();
    let c = Complex::new(
        MIN.real + (coord.x as f64 % 1.0) * (MAX.real - MIN.real),
        MIN.im + (1.0 - (coord.y as f64 % 1.0)) * (MAX.im - MIN.im),
    );

    for i in 0..MAX_ITERATIONS {
        z.square_mut();
        z.add_mut(&c);

        if z.dot() > 400.0 {
            let smooth_i = i as f64 + 1.0 - (z.abs().ln() / 4.0_f64.ln()).ln();

            let hue = 250.0 + 360.0 * (smooth_i as f32 / MAX_ITERATIONS as f32);
            let saturation = 1.0;
            let value = 1.0;

            return Color::new_hsv(hue, saturation, value);
        }
    }

    Color::black()
}
