use geometry::Transformable;
use math;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Transform {
    Translate { value: [f32; 3] },
    Scale { value: [f32; 3] },
    RotateX { value: f32 },
    RotateY { value: f32 },
    RotateZ { value: f32 },
}

impl Transform {
    pub fn to_transform(&self) -> math::Transform {
        match *self {
            Transform::Translate { value } => {
                math::Transform::new(math::Matrix4::translate(value[0], value[1], value[2]))
            }
            Transform::Scale { value } => {
                math::Transform::new(math::Matrix4::scale(value[0], value[1], value[2]))
            }
            Transform::RotateX { value } => math::Transform::new(math::Matrix4::rot_x(value)),
            Transform::RotateY { value } => math::Transform::new(math::Matrix4::rot_y(value)),
            Transform::RotateZ { value } => math::Transform::new(math::Matrix4::rot_z(value)),
        }
    }

    pub fn perform(&self, transformable: &mut dyn Transformable) {
        let transform = self.to_transform();

        transformable.transform(&transform);
    }
}
