use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum RaycastSystem {
    BuildRays,
    UpdateRaycast,
    UpdateDebugCursor,
}
