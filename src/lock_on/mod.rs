pub mod event;

use bevy::prelude::*;

use crate::{
    raycast::{RayCastMesh, RayCastSource},
    tag::MyRaycastSet,
};

use self::event::LockOnEvent;

pub struct LockOnPlugin;

impl Plugin for LockOnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LockOnEvent>().add_system(handle_lock_on);
    }
}

fn handle_lock_on(
    keys: Res<Input<KeyCode>>,
    query: Query<&mut RayCastSource<MyRaycastSet>>,
    mut lock_on_events_writer: EventWriter<LockOnEvent>,
    // mut raycast_meshes: Query<(&Name, &mut Visibility), With<RayCastMesh<MyRaycastSet>>>,
) {
    if keys.pressed(KeyCode::RControl) {
        let source = query.single();
        if let Some((entity, _)) = source.intersections.iter().next() {
            lock_on_events_writer.send(LockOnEvent::Attached(*entity));
            // if let Ok((name, _)) = raycast_meshes.get_mut(*entity) {
            //     println!("Lock-on to {}!", name.as_str());
            // }
        }
    }
    if keys.pressed(KeyCode::O) {
        lock_on_events_writer.send(LockOnEvent::Released);
    }
}
