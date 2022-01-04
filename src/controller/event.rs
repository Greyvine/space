use std::ops::Deref;
use bevy::prelude::*;

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
