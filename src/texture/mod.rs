use color::Color;
use math::Point2;
use std::fmt::Debug;

mod procedural;
mod solid;

pub use self::procedural::Procedural;
pub use self::solid::Solid;

pub type TextureCoord = Point2;

pub trait Texture: Debug {
    fn lookup(&self, uv: &TextureCoord) -> Color;
}
