use crate::color::Color;

use super::TextureCoord;

const SCALE: f32 = 5.0;
const ROTATION: f32 = std::f32::consts::PI / 4.0;

fn modulo(v: f32) -> f32 {
    v - v.floor()
}

pub fn checkerboard(coord: TextureCoord) -> Color {
    let s = coord.x * ROTATION.cos() - coord.y * ROTATION.sin();
    let t = coord.y * ROTATION.cos() + coord.x * ROTATION.sin();

    let s_v = modulo(s * SCALE) < 0.5;
    let t_v = modulo(t * SCALE) < 0.5;

    match (s_v, t_v) {
        (true, false) => Color::black(),
        (false, true) => Color::black(),
        _ => Color::white(),
    }
}
