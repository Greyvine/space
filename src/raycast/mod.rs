pub mod compute_ray;
pub mod event;
pub mod label;
pub mod mesh;
pub mod method;
pub mod primitives;
pub mod ray;
pub mod source;
pub mod state;
pub mod update_raycast;

use bevy::prelude::*;
use std::marker::PhantomData;

pub use mesh::*;
pub use method::*;
pub use source::*;

use compute_ray::*;
use label::*;
use state::*;
use update_raycast::*;

use self::event::HoverEvent;

pub struct RaycastPlugin<T: 'static + Send + Sync>(pub PhantomData<T>);

impl<T: 'static + Send + Sync> Plugin for RaycastPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultPluginState<T>>()
            .add_event::<HoverEvent>()
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(
                        compute_ray::<T>
                            .label(RaycastSystem::BuildRays)
                            .with_run_criteria(|state: Res<DefaultPluginState<T>>| {
                                state.compute_ray
                            }),
                    )
                    .with_system(
                        update_raycast::<T>
                            .label(RaycastSystem::UpdateRaycast)
                            .with_run_criteria(|state: Res<DefaultPluginState<T>>| {
                                state.update_raycast
                            })
                            .after(RaycastSystem::BuildRays),
                    ),
            );
    }
}

impl<T: 'static + Send + Sync> Default for RaycastPlugin<T> {
    fn default() -> Self {
        RaycastPlugin(PhantomData::<T>)
    }
}
