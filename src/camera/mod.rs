mod event;
mod look_direction;
mod look_entity;
mod mouse_settings;

pub mod tag;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::lock_on::event::LockOnEvent;
use crate::raycast::RayCastMesh;
use crate::tag::MyRaycastSet;

use self::event::*;
use self::mouse_settings::MouseSettings;
use self::tag::*;

pub use self::look_direction::*;
pub use self::look_entity::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseSettings>()
            .add_event::<RotationEvent>()
            .add_system(handle_mouse_input.system())
            .add_system(handle_rotation_events.system())
            .add_system(update_look_direction)
            .add_system(handle_lock_on_events);
    }
}

const PITCH_BOUND: f32 = std::f32::consts::FRAC_PI_2 - 1E-3;

fn handle_mouse_input(
    mut settings: ResMut<MouseSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rotation_events: EventWriter<RotationEvent>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion_events.iter() {
        delta -= motion.delta;
    }

    if delta.length_squared() > 1E-6 {
        delta *= settings.sensitivity;
        settings.yaw_pitch_roll += delta.extend(0.0);
        if settings.yaw_pitch_roll.y > PITCH_BOUND {
            settings.yaw_pitch_roll.y = PITCH_BOUND;
        }
        if settings.yaw_pitch_roll.y < -PITCH_BOUND {
            settings.yaw_pitch_roll.y = -PITCH_BOUND;
        }
        rotation_events.send(RotationEvent::new(Vec2::new(
            settings.yaw_pitch_roll.x,
            settings.yaw_pitch_roll.y,
        )));
    }
}

fn handle_rotation_events(
    mut events: EventReader<RotationEvent>,
    mut query: Query<&mut Transform, With<CameraTag>>,
) {
    if let Some(event) = events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = **event;
            let rotation_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 2.25, 15.0));
        }
    }
}

fn update_look_direction(
    mut events: EventReader<RotationEvent>,
    mut query: Query<&mut LookDirection>,
) {
    if let Some(event) = events.iter().next() {
        for mut look in query.iter_mut() {
            let rotation_matrix = **event;
            look.forward = rotation_matrix * -Vec3::Z;
            look.right = rotation_matrix * Vec3::X;
            look.up = rotation_matrix * Vec3::Y;
        }
    }
}

fn handle_lock_on_events(
    mut lock_on_events_reader: EventReader<LockOnEvent>,
    mut rotation_events_writer: EventWriter<RotationEvent>,
    mut raycast_meshes: Query<(&Name, &mut Transform), With<RayCastMesh<MyRaycastSet>>>,
) {
    for event in lock_on_events_reader.iter() {
        if let LockOnEvent::Attached(entity) = event {
            if let Ok((name, transform)) = raycast_meshes.get_mut(*entity) {
                let rotation = Transform::default()
                    .looking_at(transform.translation, Vec3::Y)
                    .rotation;
                rotation_events_writer.send(RotationEvent::from(rotation));
                println!("Lock-on to {}!", name.as_str());
            }
        }
    }
}
