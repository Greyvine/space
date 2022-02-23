use bevy::math::{Mat4, Vec3, Vec3A};

/// A 3D ray, with an origin and direction. The direction is guaranteed to be normalized.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Ray3d {
    pub(crate) origin: Vec3A,
    pub(crate) direction: Vec3A,
}

impl Ray3d {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray3d {
            origin: origin.into(),
            direction: direction.normalize().into(),
        }
    }

    pub fn direction(self: Ray3d) -> Vec3 {
        self.direction.into()
    }

    pub fn position(&self, distance: f32) -> Vec3 {
        (self.origin + self.direction * distance).into()
    }
}

impl From<Mat4> for Ray3d {
    fn from(transform: Mat4) -> Self {
        let pick_position_ndc = Vec3::from([0.0, 0.0, -1.0]);
        let pick_position = transform.project_point3(pick_position_ndc);
        let (_, _, source_origin) = transform.to_scale_rotation_translation();
        let ray_direction = pick_position - source_origin;
        Ray3d {
            origin: source_origin.into(),
            direction: ray_direction.normalize().into(),
        }
    }
}
