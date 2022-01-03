use std::ops::Deref;
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct RotationEvent {
    value: Vec2,
}

impl RotationEvent {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

impl Deref for RotationEvent {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
