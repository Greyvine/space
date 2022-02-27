use bevy::prelude::*;

#[derive(Component, Default)]
pub(crate) struct LockOnState {
    pub target: Option<Entity>,
    pub player_transform: Transform,
}
