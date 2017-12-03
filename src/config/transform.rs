use geometry::Transformable;
use math;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Transform {
    Translate { value: [f32; 3] },
    Scale { value: [f32; 3] },
}

impl Transform {
    pub fn perform(&self, transformable: &mut Transformable) {
        let transform = match *self {
            Transform::Translate { value } => {
                math::Transform::new(math::Matrix4::translate(value[0], value[1], value[2]))
            }
            Transform::Scale { value } => {
                math::Transform::new(math::Matrix4::scale(value[0], value[1], value[2]))
            }
        };

        transformable.transform(&transform);
    }
}
