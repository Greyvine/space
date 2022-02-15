mod event;
pub mod tag;

use crate::camera::LookDirection;
use crate::camera::LookEntity;
use crate::tag::PlayerModelTag;
use crate::HANDLE_INPUT_SYSTEM;

use self::event::*;
use self::tag::*;
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RotationEvent>()
            .add_event::<TranslationEvent>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_input.label(HANDLE_INPUT_SYSTEM),
            )
            .add_system(handle_translation_events)
            .add_system(handle_rotation_events);
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    look_direction_query: Query<&LookDirection>,
    mut controller_query: Query<&LookEntity>,
    mut translation_events: EventWriter<TranslationEvent>,
    mut rotation_events: EventWriter<RotationEvent>,
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

        let mut key_pressed = false;

        let mut desired_velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            desired_velocity += forward;
            key_pressed = true;
        }
        if keys.pressed(KeyCode::S) {
            desired_velocity -= forward;
            key_pressed = true;
        }
        if keys.pressed(KeyCode::D) {
            desired_velocity += right;
            key_pressed = true;
        }
        if keys.pressed(KeyCode::A) {
            desired_velocity -= right;
            key_pressed = true;
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

            let to_rotation = Transform::default().looking_at(forward, up).rotation;

            rotation_events.send(RotationEvent::new(&to_rotation));
            translation_events.send(TranslationEvent::new(&desired_velocity))
        }
    }
}

fn handle_translation_events(
    mut events: EventReader<TranslationEvent>,
    mut query: Query<&mut Transform, With<ControllerPlayerTag>>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.translation += **event;
        }
    }
}

fn handle_rotation_events(
    time: Res<Time>,
    mut events: EventReader<RotationEvent>,
    mut query: Query<&mut Transform, With<PlayerModelTag>>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = transform
                .rotation
                .slerp(**event, 10.0 * time.delta_seconds());
        }
    }
}
