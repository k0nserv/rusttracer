use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use image;
use image::GenericImage;

use super::{Texture, TextureCoord};
use color::Color;

#[derive(Debug)]
pub struct FileError {
    description: String,
    cause: image::ImageError,
}

impl From<image::ImageError> for FileError {
    fn from(cause: image::ImageError) -> Self {
        let description = String::from(cause.description());
        FileError { cause, description }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to load texture with error: {}", self.cause)
    }
}

impl Error for FileError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}

pub struct File {
    image: image::DynamicImage,
    path: PathBuf,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self, FileError> {
        let image = image::open(&path)?;
        Ok(File { path, image })
    }
}

impl Texture for File {
    fn lookup(&self, uv: &TextureCoord) -> Color {
        // assert!(
        //     uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.y <= 1.0,
        //     "Incorrect uv coordinate: {:?}",
        //     uv
        // );
        let (bounded_u, boundex_v) = (uv.x.abs() % 1.0, uv.y.abs() % 1.0);
        let (width, height) = self.image.dimensions();
        let x = (f64::from(bounded_u) * ((width - 1) as f64)).round() as u32;
        let y = (f64::from(boundex_v) * ((height - 1) as f64)).round() as u32;

        let pixel = self.image.get_pixel(x, y);

        Color::new(pixel[0], pixel[1], pixel[2])
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File {{ path: {:?} }}", self.path)
    }
}
