use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct TranslationEvent {
    value: Vec3,
}

impl TranslationEvent {
    pub fn new(value: &Vec3) -> Self {
        Self { value: *value }
    }
}

impl Deref for TranslationEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Default)]
pub struct ControllerRotationEvent {
    value: Quat,
}

impl ControllerRotationEvent {
    pub fn new(value: &Quat) -> Self {
        Self { value: *value }
    }
}

impl Deref for ControllerRotationEvent {
    type Target = Quat;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
