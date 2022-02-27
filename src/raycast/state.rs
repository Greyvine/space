use bevy::{ecs::schedule::ShouldRun, prelude::*};
use std::marker::PhantomData;

#[derive(Component)]
pub struct DefaultPluginState<T> {
    pub compute_ray: ShouldRun,
    pub update_raycast: ShouldRun,
    pub update_debug_cursor: bool,
    _marker: PhantomData<T>,
}

impl<T> Default for DefaultPluginState<T> {
    fn default() -> Self {
        DefaultPluginState {
            compute_ray: ShouldRun::Yes,
            update_raycast: ShouldRun::Yes,
            update_debug_cursor: false,
            _marker: PhantomData::<T>::default(),
        }
    }
}
