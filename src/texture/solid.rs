use super::{Texture, TextureCoord};
use color::Color;

#[derive(Debug, Clone)]
pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Self {
        Solid { color }
    }
}

impl Texture for Solid {
    fn lookup(&self, _uv: TextureCoord) -> Color {
        self.color
    }
}

#[cfg(test)]
mod test {
    use super::{Color, Solid, Texture, TextureCoord};

    #[test]
    fn test_lookup() {
        let c = Color::new(255, 127, 53);
        let t = Solid::new(c);
        let c1 = TextureCoord::new(0.0, 1.0);
        let c2 = TextureCoord::new(1.0, 0.0);
        let c3 = TextureCoord::new(0.0, 0.0);

        assert_eq!(t.lookup(&c1), c);
        assert_eq!(t.lookup(&c2), c);
        assert_eq!(t.lookup(&c3), c);
    }
}
