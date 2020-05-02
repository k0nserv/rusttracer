use super::TextureCoord;
use color::Color;
use math::Complex;

const MAX: Complex = Complex::new(2.0, 2.0);
const MIN: Complex = Complex::new(-2.0, -2.0);
const MAX_ITERATIONS: usize = 200;

pub fn julia(coord: TextureCoord) -> Color {
    let c = Complex::new(-0.8, 0.156);
    let mut z = Complex::new(
        MIN.real + (coord.x as f64 % 1.0) * (MAX.real - MIN.real),
        MIN.im + (1.0 - (coord.y as f64 % 1.0)) * (MAX.im - MIN.im),
    );

    for i in 0..MAX_ITERATIONS {
        z.square_mut();
        z.add_mut(&c);

        if z.dot() > 4.0 {
            let smooth_i = i as f64 + 1.0 - (z.abs().log2() / 2.0_f64.log2()).log2();

            let hue = 250.0 + 360.0 * (smooth_i as f32 / MAX_ITERATIONS as f32);
            let saturation = 1.0;
            let value = 1.0;

            return Color::new_hsv(hue, saturation, value);
        }
    }

    Color::black()
}
