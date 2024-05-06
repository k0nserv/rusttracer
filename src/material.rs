use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use serde::Deserialize;

use crate::color::Color;
use crate::texture::{Texture, TextureCoord};

#[derive(Debug)]
pub struct IllumninationModelParsingError {
    invalid_model: u8,
}

impl IllumninationModelParsingError {
    fn new(invalid_model: u8) -> Self {
        Self { invalid_model }
    }
}

impl fmt::Display for IllumninationModelParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to parse illumination model: {}",
            self.invalid_model
        )
    }
}

impl Error for IllumninationModelParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// See http://paulbourke.net/dataformats/mtl/
#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum IllumninationModel {
    /// Mode 0, constant diffuse color
    Constant = 0,
    /// Mode 1, diffuse lambertian shading
    Diffuse = 1,
    /// Mode 2, diffuse specular using lambertian shading
    /// and Blinn's Phong
    DiffuseSpecular = 2,
    /// Mode 3, diffuse specular with reflection using lambertian shading, Blinn's Phong and
    /// reflection.
    DiffuseSpecularReflective = 3,
    /// Mode 4, diffuse specular with reflection emulating glass using lambertian shading, Blinn's Phong and
    /// reflection.
    DiffuseSpecularReflectiveGlass = 4,
    /// Mode 5, diffuse specular with fresnel reflection using lambertian shading, Blinn's Phong and
    /// fresnel reflection.
    DiffuseSpecularFresnel = 5,
    /// Mode 6, diffuse specular using lambertian shading, Blinn's Phong and
    /// supporting refraction.
    DiffuseSpecularRefracted = 6,
    /// Mode 7, diffuse specular with reflections using lambertian shading, Blinn's Phong and
    /// Fresnel refraction/reflection.
    DiffuseSpecularRefractedFresnel = 7,
}

impl TryFrom<u8> for IllumninationModel {
    type Error = IllumninationModelParsingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IllumninationModel::Constant),
            1 => Ok(IllumninationModel::Diffuse),
            2 => Ok(IllumninationModel::DiffuseSpecular),
            3 => Ok(IllumninationModel::DiffuseSpecularReflective),
            4 => Ok(IllumninationModel::DiffuseSpecularReflectiveGlass),
            5 => Ok(IllumninationModel::DiffuseSpecularFresnel),
            6 => Ok(IllumninationModel::DiffuseSpecularRefracted),
            7 => Ok(IllumninationModel::DiffuseSpecularRefractedFresnel),
            _ => Err(IllumninationModelParsingError::new(value)),
        }
    }
}

pub type OptionalTexture = Option<Rc<dyn Texture>>;

#[derive(Debug)]
pub struct Material {
    pub ambient_color: Color,
    pub ambient_texture: OptionalTexture,
    pub diffuse_color: Color,
    pub diffuse_texture: OptionalTexture,
    pub specular_color: Color,
    pub specular_texture: OptionalTexture,
    pub specular_exponent: f32,
    pub illumination_model: IllumninationModel,
    pub reflection_coefficient: Option<f32>,
    pub refraction_coefficient: Option<f32>,
}

impl Material {
    pub fn is_reflective(&self) -> bool {
        self.reflection_coefficient.is_some()
            && (self.illumination_model == IllumninationModel::DiffuseSpecularReflective
                || self.illumination_model == IllumninationModel::DiffuseSpecularReflectiveGlass
                || self.illumination_model == IllumninationModel::DiffuseSpecularRefractedFresnel)
    }

    pub fn is_refractive(&self) -> bool {
        self.refraction_coefficient.is_some()
            && (self.illumination_model == IllumninationModel::DiffuseSpecularRefracted
                || self.illumination_model == IllumninationModel::DiffuseSpecularRefractedFresnel)
    }

    pub fn new(
        ambient_color: Color,
        diffuse_color: Color,
        specular_color: Color,
        specular_exponent: f32,
        illumination_model: IllumninationModel,
        reflection_coefficient: Option<f32>,
        refraction_coefficient: Option<f32>,
    ) -> Self {
        Material {
            ambient_color,
            ambient_texture: None,
            diffuse_color,
            diffuse_texture: None,
            specular_color,
            specular_texture: None,
            specular_exponent,
            illumination_model,
            reflection_coefficient,
            refraction_coefficient,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_textures(
        ambient_color: Color,
        ambient_texture: OptionalTexture,
        diffuse_color: Color,
        diffuse_texture: OptionalTexture,
        specular_color: Color,
        specular_texture: OptionalTexture,
        specular_exponent: f32,
        illumination_model: IllumninationModel,
        reflection_coefficient: Option<f32>,
        refraction_coefficient: Option<f32>,
    ) -> Self {
        Material {
            ambient_color,
            ambient_texture,
            diffuse_color,
            diffuse_texture,
            specular_color,
            specular_texture,
            specular_exponent,
            illumination_model,
            reflection_coefficient,
            refraction_coefficient,
        }
    }

    pub fn ambient_color(&self, uv: Option<TextureCoord>) -> Color {
        match &self.ambient_texture {
            None => self.ambient_color,
            Some(texture) => uv.map_or(self.ambient_color, |coord| {
                self.ambient_color * texture.lookup(coord)
            }),
        }
    }

    pub fn diffuse_color(&self, uv: Option<TextureCoord>) -> Color {
        match &self.diffuse_texture {
            None => self.diffuse_color,
            Some(texture) => uv.map_or(self.diffuse_color, |coord| {
                self.diffuse_color * texture.lookup(coord)
            }),
        }
    }

    pub fn specular_color(&self, uv: Option<TextureCoord>) -> Color {
        match &self.specular_texture {
            None => self.specular_color,
            Some(texture) => uv.map_or(self.specular_color, |coord| {
                self.specular_color * texture.lookup(coord)
            }),
        }
    }
}

pub struct MaterialTemplate {
    ambient_color: Color,
    diffuse_color: Color,
    specular_color: Color,
    specular_exponent: f32,
    illumination_model: IllumninationModel,
    reflection_coefficient: Option<f32>,
    refraction_coefficient: Option<f32>,
}

impl MaterialTemplate {
    pub fn build_material<F>(&self, builder_closure: F) -> Material
    where
        F: Fn(&mut Material),
    {
        let mut material = Material::new(
            self.ambient_color,
            self.diffuse_color,
            self.specular_color,
            self.specular_exponent,
            self.illumination_model,
            self.reflection_coefficient,
            self.refraction_coefficient,
        );

        builder_closure(&mut material);

        material
    }

    pub fn new(
        ambient_color: Color,
        diffuse_color: Color,
        specular_color: Color,
        specular_exponent: f32,
        illumination_model: IllumninationModel,
        reflection_coefficient: Option<f32>,
        refraction_coefficient: Option<f32>,
    ) -> MaterialTemplate {
        MaterialTemplate {
            ambient_color,
            diffuse_color,
            specular_color,
            illumination_model,
            specular_exponent,
            reflection_coefficient,
            refraction_coefficient,
        }
    }
}
