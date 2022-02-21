use bevy::{math::Vec3A, prelude::*};

// Holds computed intersection information
#[derive(Debug, PartialEq, Copy, Clone, Component)]
pub struct Intersection {
    position: Vec3,
    normal: Vec3,
    distance: f32,
    triangle: Option<Triangle>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub v0: Vec3A,
    pub v1: Vec3A,
    pub v2: Vec3A,
}
