use std::fmt;
use std::iter::Iterator;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    next: NextColor,
}

#[derive(Debug, Copy, Clone)]
pub enum NextColor {
    Red,
    Green,
    Blue,
    Done,
}

impl Iterator for Color {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            NextColor::Red => {
                self.next = NextColor::Green;
                Some(self.r)
            }
            NextColor::Green => {
                self.next = NextColor::Blue;
                Some(self.g)
            }
            NextColor::Blue => {
                self.next = NextColor::Done;
                Some(self.b)
            }
            NextColor::Done => None,
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
            next: NextColor::Red,
        }
    }

    pub fn new_f32(r: f32, g: f32, b: f32) -> Color {
        Color::new(
            Color::clamp((r * 255.0) as i32),
            Color::clamp((g * 255.0) as i32),
            Color::clamp((b * 255.0) as i32),
        )
    }

    pub fn new_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        let normalized_hue = hue % 360.0;
        let normalized_saturation = saturation.min(1.0).max(0.0);
        let normalized_value = value.min(1.0).max(0.0);

        let chroma = normalized_saturation * normalized_value;
        let hue_d = normalized_hue / 60.0;
        let x = chroma * (1.0 - ((hue_d % 2.0) - 1.0).abs());

        let color = match hue_d {
            v if v >= 0.0 && v <= 1.0 => (chroma, x, 0.0),
            v if v > 1.0 && v <= 2.0 => (x, chroma, 0.0),
            v if v > 2.0 && v <= 3.0 => (0.0, chroma, x),
            v if v > 3.0 && v <= 4.0 => (0.0, x, chroma),
            v if v > 4.0 && v <= 5.0 => (x, 0.0, chroma),
            v if v > 5.0 && v <= 6.0 => (chroma, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let m = normalized_value - chroma;
        Color::new_f32(color.0 + m, color.1 + m, color.2 + m)
    }

    #[inline(always)]
    pub fn r(self) -> u8 {
        self.r
    }

    #[inline(always)]
    pub fn r_f32(self) -> f32 {
        f32::from(self.r()) / 255.0
    }

    #[inline(always)]
    pub fn g(self) -> u8 {
        self.g
    }

    #[inline(always)]
    pub fn g_f32(self) -> f32 {
        f32::from(self.g()) / 255.0
    }

    #[inline(always)]
    pub fn b(self) -> u8 {
        self.b
    }

    #[inline(always)]
    pub fn b_f32(self) -> f32 {
        f32::from(self.b()) / 255.0
    }

    #[inline(always)]
    pub fn as_u32(self) -> u32 {
        0xFF00_0000 & u32::from(self.r) & u32::from(self.g) << 8 & u32::from(self.b) << 16
    }

    fn clamp(value: i32) -> u8 {
        match value {
            v if v < 0 => 0,
            v if v > i32::from(u8::max_value()) => u8::max_value(),
            v => v as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r(), self.g(), self.b())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for Color {}

impl From<[f32; 3]> for Color {
    fn from(values: [f32; 3]) -> Self {
        Self::new(
            Self::clamp((values[0] * 255.0) as i32),
            Self::clamp((values[1] * 255.0) as i32),
            Self::clamp((values[2] * 255.0) as i32),
        )
    }
}

// Math

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let f = |f: fn(Color) -> u8| Color::clamp(i32::from(f(self)) + i32::from(f(other)));

        Color::new(f(Color::r), f(Color::g), f(Color::b))
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        let r = Color::clamp(i32::from(self.r()) - i32::from(other.r()));
        let g = Color::clamp(i32::from(self.g()) - i32::from(other.g()));
        let b = Color::clamp(i32::from(self.b()) - i32::from(other.b()));

        Color::new(r, g, b)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r = self.r_f32() * other.r_f32();
        let g = self.g_f32() * other.g_f32();
        let b = self.b_f32() * other.b_f32();

        Color::new_f32(r, g, b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        let r = self.r_f32() * other;
        let g = self.g_f32() * other;
        let b = self.b_f32() * other;

        Color::new_f32(r, g, b)
    }
}

// Factory methods for common colors
macro_rules! define_color {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        #[inline(always)]
        pub fn $name() -> Color {
            Color::new($r, $g, $b)
        }
    };
}

impl Color {
    define_color!(black, 0, 0, 0);
    define_color!(white, 0xFF, 0xFF, 0xFF);
    define_color!(red, 0xFF, 0, 0);
    define_color!(green, 0, 0xFF, 0);
    define_color!(blue, 0, 0, 0xFF);
}
