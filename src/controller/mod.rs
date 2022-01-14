mod event;
pub mod tag;

use crate::HANDLE_INPUT_SYSTEM;
use crate::camera::LookDirection;
use crate::camera::LookEntity;

use self::event::*;
use self::tag::*;
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TranslationEvent>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_input.label(HANDLE_INPUT_SYSTEM),
            )
            .add_system(handle_translation_events.system());
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    look_direction_query: Query<&LookDirection>,
    mut controller_query: Query<&LookEntity>,
    mut translation_events: EventWriter<TranslationEvent>,
) {
    let xz = Vec3::new(1.0, 0.0, 1.0);
    for look_entity in controller_query.iter_mut() {
        let look = look_direction_query
            .get_component::<LookDirection>(look_entity.0)
            .expect("Failed to get LookDirection from Entity");

        let (forward, right, up) = (
            (look.forward * xz).normalize(),
            (look.right * xz).normalize(),
            Vec3::Y,
        );

        let mut desired_velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            desired_velocity += forward;
        }
        if keys.pressed(KeyCode::S) {
            desired_velocity -= forward;
        }
        if keys.pressed(KeyCode::D) {
            desired_velocity += right;
        }
        if keys.pressed(KeyCode::A) {
            desired_velocity -= right;
        }
        if keys.pressed(KeyCode::Q) {
            desired_velocity += up;
        }
        if keys.pressed(KeyCode::E) {
            desired_velocity -= up;
        }

        let speed = if keys.pressed(KeyCode::LShift) {
            2000.0
        } else {
            0.5
        };

        desired_velocity *= speed;

        translation_events.send(TranslationEvent::new(&desired_velocity))
    }

}

fn handle_translation_events(
    mut events: EventReader<TranslationEvent>,
    mut query: Query<&mut Transform, With<ControllerPlayerTag>>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.translation += **event;
            // println!("e {}", **event);
        }
    }
}
