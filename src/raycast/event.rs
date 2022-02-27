use bevy::prelude::Entity;

#[derive(Debug)]
pub enum HoverEvent {
    JustEntered(Entity),
    JustLeft(Entity),
}
