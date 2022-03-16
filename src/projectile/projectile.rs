use bevy::prelude::*;

use crate::raycast::ray::Ray3d;

#[derive(Component)]
pub struct Projectile {
    pub ballistic: bool,
    pub velocity: Vec3,
    pub direction: Vec3,
    pub ray: Ray3d,
    pub speed: f32,
}

#[derive(Component)]
pub struct Bullet;
