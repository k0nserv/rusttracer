use std::fmt::Debug;

use crate::color::Color;
use crate::math::Point2;

pub mod file;
pub mod procedural;
mod solid;

pub use self::file::File;
pub use self::procedural::Procedural;
pub use self::solid::Solid;

pub type TextureCoord = Point2;

pub trait Texture: Debug {
    fn lookup(&self, uv: TextureCoord) -> Color;
}
