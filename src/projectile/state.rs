use bevy::prelude::*;

use super::projectile::Projectile;

#[derive(Component, Default)]
pub(crate) struct DefaultPluginState {
    pub projectiles: Vec<Projectile>,
}
