use bevy::prelude::*;

pub struct MouseSettings {
    pub sensitivity: f32,
    pub yaw_pitch_roll: Vec3,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.001,
            yaw_pitch_roll: Vec3::ZERO,
        }
    }
}
