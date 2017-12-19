use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::iter::Iterator;

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

    pub fn new_from_slice(slice: [f32; 3]) -> Color {
        Self::new_f32(slice[0], slice[1], slice[2])
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.r
    }

    #[inline(always)]
    pub fn r_f32(&self) -> f32 {
        self.r() as f32 / 255.0
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.g
    }

    #[inline(always)]
    pub fn g_f32(&self) -> f32 {
        self.g() as f32 / 255.0
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.b
    }

    #[inline(always)]
    pub fn b_f32(&self) -> f32 {
        self.b() as f32 / 255.0
    }

    #[inline(always)]
    pub fn as_u32(&self) -> u32 {
        0xFF00_0000 & (self.r as u32) & (self.g as u32) << 8 & (self.b as u32) << 16
    }

    fn clamp(value: i32) -> u8 {
        match value {
            v if v < 0 => 0,
            v if v > (u8::max_value() as i32) => u8::max_value(),
            v => v as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r(), self.g(), self.b())
    }
}

// Math

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let f = |f: fn(&Color) -> u8| Color::clamp((f(&self) as i32) + (f(&other) as i32));

        Color::new(f(Color::r), f(Color::g), f(Color::b))
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        let r = Color::clamp((self.r() as i32) - (other.r() as i32));
        let g = Color::clamp((self.g() as i32) - (other.g() as i32));
        let b = Color::clamp((self.b() as i32) - (other.b() as i32));

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

// Factor methods for common colors
macro_rules! define_color {
    ($name: ident, $r: expr, $g: expr, $b: expr) => (
        #[inline(always)]
        pub fn $name() -> Color {
            Color::new($r, $g, $b)
        }
    )
}

impl Color {
    define_color!(black, 0, 0, 0);
    define_color!(white, 0xFF, 0xFF, 0xFF);
    define_color!(red, 0xFF, 0, 0);
    define_color!(green, 0, 0xFF, 0);
    define_color!(blue, 0, 0, 0xFF);
}
