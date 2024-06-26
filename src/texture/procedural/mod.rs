mod checkerboard;
mod julia;
mod mandelbrot;

use std::clone::Clone;
use std::fmt;

use crate::color::Color;

use super::{Texture, TextureCoord};

#[derive(Clone)]
pub struct Procedural<F> {
    callback: F,
}

impl<F> Procedural<F>
where
    F: Fn(TextureCoord) -> Color,
{
    pub fn new(callback: F) -> Self {
        Procedural { callback }
    }
}

impl<F> Texture for Procedural<F>
where
    F: Fn(TextureCoord) -> Color,
{
    fn lookup(&self, uv: TextureCoord) -> Color {
        (self.callback)(uv)
    }
}

impl<F> fmt::Debug for Procedural<F>
where
    F: Fn(TextureCoord) -> Color,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Procedural {{ }}")
    }
}

impl Procedural<fn(TextureCoord) -> Color> {
    pub fn mandelbrot() -> Self {
        Self::new(mandelbrot::mandelbrot)
    }

    pub fn julia() -> Self {
        Self::new(julia::julia)
    }

    pub fn checkerboard() -> Self {
        Self::new(checkerboard::checkerboard)
    }
}

#[cfg(test)]
mod test {
    use super::{Color, Procedural, Texture, TextureCoord};

    #[test]
    fn test_lookup() {
        let t = Procedural::new(|uv| Color::new_f32(uv.x, uv.y, 0.0));
        let c1 = TextureCoord::new(0.0, 1.0);
        let c2 = TextureCoord::new(1.0, 0.0);
        let c3 = TextureCoord::new(0.0, 0.0);

        assert_eq!(t.lookup(c1), Color::new(0, 255, 0));
        assert_eq!(t.lookup(c2), Color::new(255, 0, 0));
        assert_eq!(t.lookup(c3), Color::new(0, 0, 0));
    }
}
