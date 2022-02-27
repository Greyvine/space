use bevy::{math::Vec3A, prelude::*};

use super::triangle::Triangle;

// Holds computed intersection information
#[derive(Debug, PartialEq, Copy, Clone, Component)]
pub struct Intersection {
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub triangle: Option<Triangle>,
}

impl Intersection {
    pub fn new(
        position: Vec3,
        normal: Vec3,
        pick_distance: f32,
        triangle: Option<Triangle>,
    ) -> Self {
        Intersection {
            position,
            normal,
            distance: pick_distance,
            triangle,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}
