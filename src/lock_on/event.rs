use bevy::prelude::Entity;

#[derive(Debug)]
pub enum LockOnEvent {
    Attached(Entity),
    Released,
}
