pub mod event;
mod state;
pub mod tag;

use crate::camera::LookDirection;
use crate::camera::LookEntity;
use crate::lock_on::event::LockOnEvent;
use crate::tag::PlayerModelTag;
use crate::HANDLE_INPUT_SYSTEM;

use self::event::*;
use self::state::LockOnState;
use self::tag::*;
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockOnState>()
            .add_event::<ControllerRotationEvent>()
            .add_event::<TranslationEvent>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_input.label(HANDLE_INPUT_SYSTEM),
            )
            .add_system(handle_translation_events)
            .add_system(handle_rotation_events)
            .add_system(handle_lock_on_events);
    }
}

const SPEED: f32 = 20.0;

fn handle_keyboard_input_new(
    keys: Res<Input<KeyCode>>,
    lock_on_state: Res<LockOnState>,
    transforms: Query<&Transform>,
    look_direction_query: Query<&LookDirection>,
    mut controller_query: Query<&LookEntity>,
    mut translation_events: EventWriter<TranslationEvent>,
    mut rotation_events: EventWriter<ControllerRotationEvent>,
) {
    if keys.any_pressed([
        KeyCode::W,
        KeyCode::A,
        KeyCode::S,
        KeyCode::D,
        KeyCode::Q,
        KeyCode::E,
    ]) {
        for look_entity in controller_query.iter_mut() {
            let xz = Vec3::new(1.0, 0.0, 1.0);
            if let Some(target) = lock_on_state.target {
                if let Ok(transform) = transforms.get(target) {
                    let direction =
                        transform.translation - lock_on_state.player_transform.translation;

                    let (forward, right, up) = (
                        (direction.z * xz).normalize(),
                        (direction.x * xz).normalize(),
                        Vec3::Y,
                    );

                    let mut clamp_direction = false;

                    let mut desired_velocity = Vec3::ZERO;
                    if keys.pressed(KeyCode::W) {
                        desired_velocity += forward;
                        clamp_direction = true;
                    }
                    if keys.pressed(KeyCode::S) {
                        desired_velocity -= forward;
                        clamp_direction = true;
                    }
                    if keys.pressed(KeyCode::D) {
                        desired_velocity += right;
                        // clamp_direction = true;
                    }
                    if keys.pressed(KeyCode::A) {
                        desired_velocity -= right;
                        // clamp_direction = true;
                    }
                    if keys.pressed(KeyCode::Q) {
                        desired_velocity += up;
                    }
                    if keys.pressed(KeyCode::E) {
                        desired_velocity -= up;
                    }

                    let speed = if keys.pressed(KeyCode::LShift) {
                        SPEED
                    } else {
                        0.5
                    };

                    desired_velocity *= speed;

                    let rotation = Transform::default().looking_at(direction, Vec3::Y).rotation;
                    rotation_events.send(ControllerRotationEvent::new(&rotation));
                    translation_events.send(TranslationEvent::new(&desired_velocity));
                }
            } else {
                let look_direction = look_direction_query
                    .get_component::<LookDirection>(look_entity.0)
                    .expect("Failed to get LookDirection from Entity");
                let (forward, right, up) = (
                    (look_direction.forward * xz).normalize(),
                    (look_direction.right * xz).normalize(),
                    Vec3::Y,
                );

                let mut clamp_direction = false;

                let mut desired_velocity = Vec3::ZERO;
                if keys.pressed(KeyCode::W) {
                    desired_velocity += forward;
                    clamp_direction = true;
                }
                if keys.pressed(KeyCode::S) {
                    desired_velocity -= forward;
                    clamp_direction = true;
                }
                if keys.pressed(KeyCode::D) {
                    desired_velocity += right;
                    // clamp_direction = true;
                }
                if keys.pressed(KeyCode::A) {
                    desired_velocity -= right;
                    // clamp_direction = true;
                }
                if keys.pressed(KeyCode::Q) {
                    desired_velocity += up;
                }
                if keys.pressed(KeyCode::E) {
                    desired_velocity -= up;
                }

                let speed = if keys.pressed(KeyCode::LShift) {
                    SPEED
                } else {
                    0.5
                };

                desired_velocity *= speed;

                if clamp_direction {
                    let rotation = Transform::default().looking_at(forward, up).rotation;
                    rotation_events.send(ControllerRotationEvent::new(&rotation));
                }
                translation_events.send(TranslationEvent::new(&desired_velocity));
            }
        }
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    lock_on_state: Res<LockOnState>,
    look_direction_query: Query<&LookDirection>,
    mut controller_query: Query<&LookEntity>,
    mut translation_events: EventWriter<TranslationEvent>,
    mut rotation_events: EventWriter<ControllerRotationEvent>,
    transforms: Query<&Transform>,
) {
    let xz = Vec3::new(1.0, 0.0, 1.0);
    for look_entity in controller_query.iter_mut() {
        let look_direction = look_direction_query
            .get_component::<LookDirection>(look_entity.0)
            .expect("Failed to get LookDirection from Entity");

        let (forward, right, up) = (
            (look_direction.forward * xz).normalize(),
            (look_direction.right * xz).normalize(),
            Vec3::Y,
        );

        let mut clamp_direction = false;
        let mut key_pressed = false;

        let mut desired_velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            desired_velocity += forward;
            key_pressed = true;
            clamp_direction = true;
        }
        if keys.pressed(KeyCode::S) {
            desired_velocity -= forward;
            key_pressed = true;
            clamp_direction = true;
        }
        if keys.pressed(KeyCode::D) {
            desired_velocity += right;
            key_pressed = true;
            // clamp_direction = true;
        }
        if keys.pressed(KeyCode::A) {
            desired_velocity -= right;
            key_pressed = true;
            // clamp_direction = true;
        }
        if keys.pressed(KeyCode::Q) {
            desired_velocity += up;
            key_pressed = true;
        }
        if keys.pressed(KeyCode::E) {
            desired_velocity -= up;
            key_pressed = true;
        }

        let speed = if keys.pressed(KeyCode::LShift) {
            2000.0
        } else {
            0.5
        };

        if key_pressed {
            desired_velocity *= speed;

            if let Some(target) = lock_on_state.target {
                if let Ok(transform) = transforms.get(target) {
                    let direction =
                        transform.translation - lock_on_state.player_transform.translation;
                    let rotation = Transform::default().looking_at(direction, up).rotation;
                    rotation_events.send(ControllerRotationEvent::new(&rotation));
                }
            } else {
                if clamp_direction {
                    let rotation = Transform::default().looking_at(forward, up).rotation;
                    rotation_events.send(ControllerRotationEvent::new(&rotation));
                }
            }

            translation_events.send(TranslationEvent::new(&desired_velocity))
        }
    }
}

fn handle_translation_events(
    mut events: EventReader<TranslationEvent>,
    mut query: Query<&mut Transform, With<ControllerPlayerTag>>,
    mut lock_on_state: ResMut<LockOnState>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.translation += **event;
            lock_on_state.player_transform = *transform;
        }
    }
}

fn handle_rotation_events(
    time: Res<Time>,
    mut events: EventReader<ControllerRotationEvent>,
    mut query: Query<&mut Transform, With<PlayerModelTag>>,
    mut lock_on_state: ResMut<LockOnState>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = transform
                .rotation
                .slerp(**event, 10.0 * time.delta_seconds());
            lock_on_state.player_transform = *transform;
        }
    }
}

fn handle_lock_on_events(
    mut lock_on_events: EventReader<LockOnEvent>,
    mut lock_on_state: ResMut<LockOnState>,
) {
    for event in lock_on_events.iter() {
        if let LockOnEvent::Attached(entity) = event {
            lock_on_state.target = Some(*entity);
        } else if let LockOnEvent::Released = event {
            println!("Release Lock-On");
            lock_on_state.target = None;
        }
    }
}
