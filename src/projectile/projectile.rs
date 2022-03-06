use bevy::prelude::*;

use crate::raycast::ray::Ray3d;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub ray: Ray3d,
}
