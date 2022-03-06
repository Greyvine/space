use bevy::{
    math::{Mat4, Vec3, Vec3A},
    render::primitives::Aabb,
};

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

    pub fn intersects_aabb(&self, aabb: &Aabb, model_to_world: &Mat4) -> Option<[f32; 2]> {
        let world_to_model = model_to_world.inverse();
        let ray_dir: Vec3A = world_to_model
            .transform_vector3(self.direction.into())
            .into();
        let ray_origin: Vec3A = world_to_model.transform_point3(self.origin.into()).into();

        let t_0: Vec3A = (Vec3A::from(aabb.min()) - ray_origin) / ray_dir;
        let t_1: Vec3A = (Vec3A::from(aabb.max()) - ray_origin) / ray_dir;
        let t_min: Vec3A = t_0.min(t_1);
        let t_max: Vec3A = t_0.max(t_1);

        let mut hit_near = t_min.x;
        let mut hit_far = t_max.x;

        if hit_near > t_max.y || t_min.y > hit_far {
            return None;
        }

        if t_min.y > hit_near {
            hit_near = t_min.y;
        }
        if t_max.y < hit_far {
            hit_far = t_max.y;
        }

        if (hit_near > t_max.z) || (t_min.z > hit_far) {
            return None;
        }

        if t_min.z > hit_near {
            hit_near = t_min.z;
        }
        if t_max.z < hit_far {
            hit_far = t_max.z;
        }
        Some([hit_near, hit_far])
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
