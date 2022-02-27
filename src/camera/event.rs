use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct RotationEvent {
    value: Quat,
}

impl RotationEvent {
    pub fn new(v: Vec2) -> Self {
        Self {
            value: Quat::from_rotation_y(v.x) * Quat::from_rotation_x(v.y),
        }
    }
}

impl From<Quat> for RotationEvent {
    fn from(value: Quat) -> Self {
        Self { value }
    }
}

impl Deref for RotationEvent {
    type Target = Quat;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
